use bevy::prelude::*;
use crate::ascii_world::AsciiTile;

#[derive(Component)]
struct PlayerMarker;


fn startup(mut commands: Commands) {
    commands.spawn((
        AsciiTile {pos: IVec3::new(0, 50, 0) },
        PlayerMarker
    ));
}
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}