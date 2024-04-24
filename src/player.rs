use bevy::prelude::*;
use crate::ascii_world::{AsciiAddEvent, AsciiTile};

#[derive(Component)]
struct PlayerMarker;


fn startup(
    mut commands: Commands,
    mut event: EventWriter<AsciiAddEvent>,
) {
    let entity = commands.spawn((
        AsciiTile {pos: UVec3::new(30, 30, 0) },
        PlayerMarker
    )).id();
    event.send(AsciiAddEvent {
        entity,
        pos: UVec3::new(30, 30, 0)
    });
}
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}