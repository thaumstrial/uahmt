use bevy::input::mouse::{MouseWheel, MouseMotion};
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::WindowResolution;
use bevy_xpbd_2d::prelude::*;

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct Engine {
    throttle: f32,
    thrust: f32,
    momentum_limit: f32,
    momentum: f32,
    enable_sas: bool,
}
#[derive(Component)]
struct PlayerMarker;
#[derive(Component)]
struct ObjectMarker;

#[derive(PartialEq, Default)]
enum CameraMode {
    Free,
    #[default]
    Follow
}
#[derive(Component, Default)]
struct CameraController {
    mode: CameraMode,   
    relative_translation: Vec3
}
#[derive(Component)]
struct Hold{
    constraint: Option<Entity>
}
impl Default for Hold {
    fn default() -> Self {
        Hold { constraint: None }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load font
    let font_hanle: Handle<Font> = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font: font_hanle.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    commands.insert_resource(UiFont(font_hanle));

    // setup camera
    commands.spawn((
        Camera2dBundle::default(),
        CameraController::default()
    ));

    // setup player
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("@", text_style.clone())
                .with_alignment(text_alignment.clone()),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        GravityScale(0.),
        Engine {
            throttle: 1.,
            thrust: 200.,
            momentum_limit: 1.,
            momentum: 10.,
            enable_sas: false
        },
        Hold::default(),
        PlayerMarker
    ));
    // setup wall
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("#", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(-50., 50., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        GravityScale(0.),
        ObjectMarker
    ));
    // setup wall
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("O", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(50., 50., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        GravityScale(0.),
        ObjectMarker
    ));
} 

fn player_controller(
    mut q_player: Query<(Entity, &Transform, &mut LinearVelocity, &mut AngularVelocity, &mut Engine, &mut Hold), With<PlayerMarker>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    q_spatial: SpatialQuery,
    q_rigidbody: Query<&Transform, With<ObjectMarker>>
) {
    if let Ok((player_id, p_transform, mut linear_v, mut angular_v, mut engine, mut hold)) = q_player.get_single_mut() {
        let delta_time = time.delta_seconds();
        // linear movement
        let mut linear_mov = Vec2::ZERO;
        if keyboard.pressed(KeyCode::A) {
            linear_mov.x -= 1.;
        }
        if keyboard.pressed(KeyCode::D) { 
            linear_mov.x += 1.;
        }
        if keyboard.pressed(KeyCode::S) { 
            linear_mov.y -= 1.;
        }
        if keyboard.pressed(KeyCode::W) {
            linear_mov.y += 1.;
        }
        linear_v.0 += linear_mov.normalize_or_zero() * engine.thrust * engine.throttle * delta_time;
        // angular movement
        if keyboard.pressed(KeyCode::Q) {
            angular_v.0 += engine.momentum * engine.momentum_limit * delta_time;
        }
        if keyboard.pressed(KeyCode::E) {
            angular_v.0 -= engine.momentum * engine.momentum_limit * delta_time;
        }
        // change engine throttle
        if keyboard.pressed(KeyCode::ShiftLeft) {
            engine.throttle = (engine.throttle + 0.1).min(1.);
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            engine.throttle = (engine.throttle - 0.1).max(0.);
        }
        if keyboard.pressed(KeyCode::X) {
            engine.throttle = 0.;
        }
        if keyboard.pressed(KeyCode::Z) {
            engine.throttle = 1.;
        }
        // toggle sas
        if keyboard.just_pressed(KeyCode::T) {
            engine.enable_sas = !engine.enable_sas;
        }
        // hold/drop entity
        if keyboard.just_pressed(KeyCode::F) {
            match hold.constraint {
                Some(_constraint) => {
                    commands.entity(_constraint).despawn();
                    hold.constraint = None;
                }
                None => {
                    let origin: Vec2 = p_transform.translation.truncate();
                    let direction = (p_transform.rotation * Vec3::new(1., 0., 0.)).truncate().normalize();

                    let hits = q_spatial.ray_hits(
                        origin,
                        direction,
                        24.,
                        2,
                        false,
                        SpatialQueryFilter::default()
                    );
                        
                    match hits.as_slice() {
                        [hit1, hit2] => {
                            let (p_hit, t_hit) = if hit1.entity == player_id {
                                (hit1, hit2) 
                            } else {
                                (hit2, hit1) 
                            };
                            if let Ok(t_transform) = q_rigidbody.get_component::<Transform>(t_hit.entity) {
                                let anchor1_vec = direction * p_hit.time_of_impact;
                                let anchor2_vec = origin + direction * t_hit.time_of_impact - t_transform.translation.truncate();
                                let joint_id = commands.spawn(
                                    FixedJoint::new(player_id, t_hit.entity)
                                        .with_local_anchor_1(anchor1_vec)
                                        .with_local_anchor_2(anchor2_vec)
                                        .with_compliance(0.0001)
                                ).id();
                                hold.constraint = Some(joint_id);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn camera_controller(
    mut q_camera: Query<(&mut OrthographicProjection, &mut Transform, &mut CameraController)>, 
    mut scroll: EventReader<MouseWheel>, 
    mut motion: EventReader<MouseMotion>,
    keyboard: Res<Input<KeyCode>>,
    button: Res<Input<MouseButton>>,
    time: Res<Time>
) {
    if let Ok((mut projection, mut c_transform, mut controller)) = q_camera.get_single_mut() {
        let delta_time = time.delta_seconds();
        // camera movement
        if button.pressed(MouseButton::Middle) {
            for ev in motion.read() {
                let motion_vec = Vec3::new(-ev.delta.x * projection.scale, ev.delta.y * projection.scale, 0.);
                match controller.mode {
                    CameraMode::Follow => {controller.relative_translation += motion_vec * delta_time * 100.;},
                    CameraMode::Free => {c_transform.translation += motion_vec * delta_time * 100.;}
                }
            }
        }
        // zoom
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll.read() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    projection.scale -= ev.y / 10.;
                }
                MouseScrollUnit::Pixel => {
                    projection.scale -= ev.y / 10.;
                }
            }
        }
        if keyboard.just_pressed(KeyCode::V) {
            controller.mode = match controller.mode {
                CameraMode::Follow => {
                    CameraMode::Free
                },
                CameraMode::Free => {
                    controller.relative_translation = Vec3::ZERO;
                    CameraMode::Follow
                }
            }
        }
    }
}

fn sync_camera(
    mut q_camera: Query<(&mut Transform, &CameraController), Without<PlayerMarker>>,
    q_player: Query<&Transform, With<PlayerMarker>>
) {
    if let (
        Ok((mut c_transform, controller)),
        Ok(p_transform)) 
        = (
        q_camera.get_single_mut(),
        q_player.get_single()
    ) {
        match controller.mode {
            CameraMode::Free => {},
            CameraMode::Follow => {
                c_transform.translation = p_transform.translation + controller.relative_translation;
            }
        }
    }
}

fn debug_controller(mut commands: Commands, physics_config: Res<PhysicsDebugConfig>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F3) {
        commands.insert_resource(PhysicsDebugConfig {
            enabled: !physics_config.enabled,
            ..*physics_config
        })
    }
}

fn activate_sas(mut q_entity: Query<(&mut AngularVelocity, &Engine, Option<&PlayerMarker>)>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    for (mut angular_v, engine, marker) in q_entity.iter_mut() {
        if engine.enable_sas ==  true {
            if let Some(_marker) = marker {
                if keyboard.any_pressed([KeyCode::Q, KeyCode::E]) {
                    continue;
                }
            }
            angular_v.0 = (angular_v.0 - engine.momentum * engine.momentum_limit * time.delta_seconds()).max(0.);
        }
    }
}

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1920., 1080.),
                    title: String::from("Uahmt"),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(PhysicsDebugConfig {
            enabled: false,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (player_controller, camera_controller, debug_controller))
        .add_systems(
            PostUpdate,
            sync_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )
        .add_systems(Update, activate_sas)
        .run();
}
