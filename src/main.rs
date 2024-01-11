use bevy::{prelude::*, window::WindowResolution};

struct Player(u64);


#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraMarker
    ));
}
fn load_font(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font_hanle: Handle<Font> = asset_server.load("font.ttf");
    commands.insert_resource(UiFont(font_hanle));
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
        .add_systems(Startup, (setup_camera, load_font))
        .run();
}
