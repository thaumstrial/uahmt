use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct AsciiRenderPlugin;
impl Plugin for AsciiRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_plugins(TilemapPlugin)
            .insert_resource(ClearColor(Color::BLACK));
    }
}