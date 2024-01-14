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
    Follow,
    Fixed
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

#[derive(Component)]
struct GravitationalField{
    g: f32,
    s: f32,
    h: f32,
    r: f32
}

#[derive(Component)]
struct Hardness(f32);

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
        Hardness(10000.),
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
        Hardness(10000.),
        ObjectMarker
    ));
    // setup wall
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("‰", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(50., 50., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        GravityScale(0.),
        Hardness(10000.),
        ObjectMarker
    ));
    // setup wall
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("÷", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(50., -50., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        ColliderDensity(10000.),
        GravityScale(0.),
        GravitationalField {
            g: 10.,
            s: 1.,
            h: 1000.,
            r: 10000.
        },
        Hardness(10000.),
        ObjectMarker
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("÷\n÷÷÷\n÷÷÷÷÷\n÷÷÷÷÷÷÷\n÷÷÷÷÷\n÷÷÷\n÷", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(-50., -50., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        ColliderDensity(10000.),
        GravityScale(0.),
        GravitationalField {
            g: 10.,
            s: 1.,
            h: 1000.,
            r: 10000.
        },
        Hardness(500.),
        ObjectMarker
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("N", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(-80., -80., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        GravityScale(0.),
        GravitationalField {
            g: 10.,
            s: 1.,
            h: 1000.,
            r: 10000.
        },
        Hardness(250.),
        ObjectMarker
    ));
} 

fn player_controller(
    mut q_player: Query<(Entity, &Transform, &mut LinearVelocity, &mut AngularVelocity, &mut Engine, &mut Hold, &Mass), With<PlayerMarker>>,
    q_camera: Query<(&Transform, &CameraController)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    q_spatial: SpatialQuery,
    mut q_rigidbody: Query<(&Transform, &mut LinearVelocity, &Mass), (With<ObjectMarker>, Without<PlayerMarker>)>
) {
    if let Ok((
        player_id, 
        p_transform, 
        mut linear_v, 
        mut angular_v, 
        mut engine, 
        mut hold,
        p_mass
    )) = q_player.get_single_mut() {
        let delta_time = time.delta_seconds();
        // linear movement
        let mut linear_mov = Vec3::ZERO;
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
        if let Ok((c_transform, c_controller)) = q_camera.get_single() {
            if c_controller.mode == CameraMode::Fixed {
                linear_mov = c_transform.rotation * linear_mov;
            }
        }
        linear_v.0 += linear_mov.truncate().normalize_or_zero() * engine.thrust * engine.throttle * delta_time;
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
                        25.,
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
                                let anchor2_vec = origin + direction * p_hit.time_of_impact - t_transform.translation.truncate();
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
        // push entity
        if keyboard.just_pressed(KeyCode::Space) {
            // optimized code has bug
            let q_spatial_filter = match hold.constraint {
                Some(_constraint) => {SpatialQueryFilter::new().without_entities([player_id]).without_entities([_constraint])},
                None => {SpatialQueryFilter::new().without_entities([player_id])},
            };
            let t_entity = match q_spatial.project_point(
                p_transform.translation.truncate(),
                true,
                q_spatial_filter
            ) {
                Some(_p_projection) => {Some(_p_projection.entity)},
                None => {None},
            };
            match t_entity {
                Some(_t_entity) => {
                    if let Ok((
                        t_transform, 
                        mut t_linear_v, 
                        t_mass
                    )) = q_rigidbody.get_mut(_t_entity) 
                    {
                        let distance = t_transform.translation.truncate().distance(p_transform.translation.truncate());
                        if distance <= 50. {
                            let direction = (t_transform.translation - p_transform.translation).normalize().truncate();
                            let delta_v = direction * engine.thrust * engine.throttle * 100. * delta_time;
                            t_linear_v.0 += delta_v * p_mass.0 / t_mass.0;
                            linear_v.0 += -delta_v   
                        }
                    }
                },
                None => {}
            };
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
                    CameraMode::Free => {c_transform.translation += motion_vec * delta_time * 100.;},
                    CameraMode::Fixed => {},
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
                    CameraMode::Fixed
                }
                CameraMode::Fixed => {
                    c_transform.rotation = Quat::IDENTITY;
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
            CameraMode::Fixed => {
                c_transform.translation = p_transform.translation.clone();
                c_transform.rotation = p_transform.rotation.clone();
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

fn process_sas(
    mut q_entity: Query<(&mut AngularVelocity, &Engine, Option<&PlayerMarker>)>, 
    keyboard: Res<Input<KeyCode>>, 
    time: Res<Time>
) {
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

fn process_gfield(
    q_planet: Query<(&Transform, &GravitationalField, &Mass, Entity)>,
    q_transform: Query<&Transform, Or<(With<ObjectMarker>, With<PlayerMarker>)>>,
    mut q_linearvelocity: Query<&mut LinearVelocity, Or<(With<ObjectMarker>, With<PlayerMarker>)>>,
    q_spatial: SpatialQuery,
    time: Res<Time>
) {
    for (p_transform, p_gfield, p_mass, p_entity) in q_planet.iter() {
        let intersections = q_spatial.shape_intersections(
            &Collider::ball(p_gfield.r), 
            p_transform.translation.truncate(), 
            0., 
            SpatialQueryFilter::default().without_entities([p_entity])
        );
        for &t_entity in intersections.iter() {
            if let (Ok(r_transform), Ok(mut linear_v)) = (q_transform.get_component::<Transform>(t_entity), q_linearvelocity.get_component_mut::<LinearVelocity>(t_entity)) {
                let distance = p_transform.translation.truncate().distance(r_transform.translation.truncate()) * p_gfield.s + p_gfield.h;
                let delta_v = p_gfield.g * p_mass.0 / distance.powf(2.) * time.delta_seconds();
                linear_v.0 += (p_transform.translation.truncate() - r_transform.translation.truncate()).normalize() * delta_v;
            }
        }
    }
}

fn process_destroy(
    mut commands: Commands,
    mut collision: EventReader<CollisionStarted>,
    q_rigidbody: Query<(&Hardness, &LinearVelocity, &Mass), Or<(With<ObjectMarker>, With<PlayerMarker>)>>,
    time: Res<Time>
) {
    for CollisionStarted(entity1, entity2) in collision.read() {
        if let (
            Ok(h1), 
            Ok(v1), 
            Ok(m1), 
            Ok(h2), 
            Ok(v2), 
            Ok(m2)) = (
                q_rigidbody.get_component::<Hardness>(*entity1), 
                q_rigidbody.get_component::<LinearVelocity>(*entity1), 
                q_rigidbody.get_component::<Mass>(*entity1), 
                q_rigidbody.get_component::<Hardness>(*entity2), 
                q_rigidbody.get_component::<LinearVelocity>(*entity2), 
                q_rigidbody.get_component::<Mass>(*entity2)
        ) {
            let vt = 2. * (m1.0 * v1.0 + m2.0 * v2.0) / (m1.0 + m2.0);
            let impulse = (vt - 2. * v1.0).length() * m1.0 * time.delta_seconds();
            if impulse >= h1.0 {
                commands.entity(*entity1).despawn();
            }
            if impulse >= h2.0 {
                commands.entity(*entity2).despawn();
            }
        };
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
        .add_systems(Update, (process_sas, process_gfield, process_destroy))
        .run();
}
