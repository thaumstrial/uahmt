use bevy::{prelude::*, window::WindowResolution};
use bevy_xpbd_2d::prelude::*;

#[derive(Bundle, Default)]
struct PlayerBundle{
    rigid_body: RigidBody,
    text: Text2dBundle
}

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct CameraMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load font
    let font_hanle: Handle<Font> = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font: font_hanle.clone(),
        font_size: 60.0,
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
    commands.spawn(PlayerBundle {
        text: Text2dBundle {
            text: Text::from_section("@", text_style)
                .with_alignment(text_alignment),
            ..default()
        },
        ..Default::default()
    });
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
        .add_systems(Startup, setup)
        .run();
}
