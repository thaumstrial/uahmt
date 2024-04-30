mod ascii_world;
mod ascii_render;
mod debug;
mod player;
mod living_entity;
mod world_map;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;

fn setup(mut commands: Commands) {

}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum MainState {
    #[default]
    MainMenu,
    InGame
}

#[bevy_main]
fn main() {
    App::new()
        .init_state::<MainState>()
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
        .add_plugins(ui::UiPlugin)
        .add_plugins(world_map::WorldMapPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(living_entity::LivingEntityPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_systems(Startup, setup)
        .run();
}
