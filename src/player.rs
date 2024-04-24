use bevy::prelude::*;
use crate::ascii_world::{AsciiAddEvent, AsciiMoveEvent, AsciiTile};
use crate::living_entity::Movement;

#[derive(Component)]
struct PlayerMarker;


fn startup(
    mut commands: Commands,
    mut event: EventWriter<AsciiAddEvent>,
) {
    let entity = commands.spawn((
        AsciiTile {pos: UVec3::new(30, 30, 0) },
        Movement {
            v: 0.1,
            d: Vec3::ZERO
        },
        PlayerMarker,
    )).id();
    event.send(AsciiAddEvent {
        entity,
        pos: UVec3::new(30, 30, 0)
    });
}

fn keyboard_input(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Movement, &mut AsciiTile), With<PlayerMarker>>
) {
    if let Ok((mut movement, mut tile))= player.get_single_mut() {
        if key.pressed(KeyCode::KeyW) {
            movement.d.y -= movement.v;
        }
        if key.pressed(KeyCode::KeyA) {
            movement.d.x -= movement.v;
        }
        if key.pressed(KeyCode::KeyS) {
            movement.d.y += movement.v;
        }
        if key.pressed(KeyCode::KeyD) {
            movement.d.x += movement.v;
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(PreUpdate, keyboard_input);
    }
}