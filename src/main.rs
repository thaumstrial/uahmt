use bevy::input::mouse::{MouseWheel, MouseMotion};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_xpbd_2d::prelude::*;

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct Engine {
    throttle: f32,
    thrust: f32
}
#[derive(Component)]
struct MomentumWheel {
    limit: f32,
    momentum: f32
}
#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct CameraMarker;

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
        CameraMarker
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
            thrust: 1.
        },
        MomentumWheel {
            limit: 1.,
            momentum: 0.02
        },
        PlayerMarker
    ));
    // setup wall
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("#", text_style.clone())
                .with_alignment(text_alignment.clone()),
            transform: Transform::from_xyz(50., 50., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20., 20.),
        GravityScale(0.)
    ));
} 

fn player_controller(mut q_player: Query<(&mut LinearVelocity, &mut AngularVelocity, &mut Engine, &MomentumWheel), With<PlayerMarker>>, keyboard: Res<Input<KeyCode>>) {
    if let Ok((mut linear_v, mut angular_v, mut engine, m_wheel)) = q_player.get_single_mut() {
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
        linear_v.0 += linear_mov.normalize_or_zero() * engine.thrust * engine.throttle;
        // angular movement
        if keyboard.pressed(KeyCode::Q) {
            angular_v.0 += m_wheel.momentum * m_wheel.limit;
        }
        if keyboard.pressed(KeyCode::E) {
            angular_v.0 -= m_wheel.momentum * m_wheel.limit;
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
    }
}

fn camera_controller(
    mut q_camera: Query<(&mut OrthographicProjection, &mut Transform), With<CameraMarker>>, 
    mut scroll: EventReader<MouseWheel>, 
    mut motion: EventReader<MouseMotion>, 
    button: Res<Input<MouseButton>>
) {
    if let Ok((mut projection, mut transform)) = q_camera.get_single_mut() {
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
        if button.pressed(MouseButton::Middle) {
            for ev in motion.read() {
                transform.translation.x -= ev.delta.x * projection.scale;
                transform.translation.y += ev.delta.y * projection.scale;
            }
        }
    }
}

fn debug_controller(mut command: Commands, physics_config: Res<PhysicsDebugConfig>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F3) {
        command.insert_resource(PhysicsDebugConfig {
            enabled: !physics_config.enabled,
            ..*physics_config
        })
    }
}

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
        .run();
}
