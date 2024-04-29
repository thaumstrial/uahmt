use bevy::prelude::*;
use crate::ascii_world::{AsciiAddEvent, AsciiMoveEvent, AsciiTile, WorldSettings};
use crate::living_entity::Movement;

#[derive(Component)]
pub struct PlayerMarker;


fn startup(
    mut commands: Commands,
    mut event: EventWriter<AsciiAddEvent>,
) {
    let entity = commands.spawn((
        AsciiTile {pos: UVec3::new(30, 30, 2) },
        Movement {
            v: 20.,
            d: Vec3::ZERO
        },
        PlayerMarker,
    )).id();
    event.send(AsciiAddEvent {
        entity,
        pos: UVec3::new(30, 30, 2)
    });
}

fn keyboard_input(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(Entity, &mut Movement, &mut AsciiTile), With<PlayerMarker>>,
    mut mov: EventWriter<AsciiMoveEvent>,
    settings: Res<WorldSettings>
) {
    if let Ok((entity ,mut movement, mut tile))= player.get_single_mut() {
        let dx =  time.delta_seconds() * movement.v;
        if key.pressed(KeyCode::KeyW) {
            movement.d.y -= dx;
        }
        if key.pressed(KeyCode::KeyA) {
            movement.d.x -= dx;
        }
        if key.pressed(KeyCode::KeyS) {
            movement.d.y += dx;
        }
        if key.pressed(KeyCode::KeyD) {
            movement.d.x += dx;
        }
        let mut new_pos = tile.pos.clone();
        if movement.d.x >= 1. {
            if new_pos.x < settings.size.x - 1 {
                new_pos.x += movement.d.x as u32;
            }
            movement.d.x = 0.;
        }
        if movement.d.x <= -1.{
            if 0 < new_pos.x {
                new_pos.x -= -movement.d.x as u32;
            }
            movement.d.x = 0.;
        }
        if movement.d.y >= 1. {
            if new_pos.y < settings.size.y - 1 {
                new_pos.y += movement.d.y as u32;
            }
            movement.d.y = 0.;
        }
        if movement.d.y <= -1. {
            if 0 < new_pos.y {
                new_pos.y -= -movement.d.y as u32;
            }
            movement.d.y = 0.;
        }
        if tile.pos != new_pos {
            mov.send(AsciiMoveEvent {
                entity,
                old_pos: tile.pos.clone(),
                new_pos
            });
            tile.pos = new_pos;
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