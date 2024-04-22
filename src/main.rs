mod ascii_world;
mod ascii_render;
mod debug;

use bevy::prelude::*;
use bevy::window::WindowResolution;

fn setup(mut commands: Commands) {
    // commands.spawn(Text2dBundle {});
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
        .add_plugins(ascii_world::AsciiWorldPlugin)
        .add_plugins(ascii_render::AsciiRenderPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_systems(Startup, setup)
        .run();
}
