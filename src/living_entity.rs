use bevy::prelude::*;
use crate::ascii_world::{AsciiMoveEvent, AsciiTile};

#[derive(Component)]
pub struct Movement {
    pub v: f32,
    pub d: Vec3
}

pub fn movement_system(
    mut q: Query<(Entity, &mut Movement, &mut AsciiTile)>,
    mut mov: EventWriter<AsciiMoveEvent>,
) {
    for (entity, mut movement, mut tile) in q.iter_mut() {
        let old_pos = tile.pos.clone();
        if movement.d.x >= 1. {
            tile.pos.x += movement.d.x as u32;
            movement.d.x = 0.;
        }
        if movement.d.x <= -1. {
            tile.pos.x -= -movement.d.x as u32;
            movement.d.x = 0.;
        }
        if movement.d.y >= 1. {
            tile.pos.y += movement.d.y as u32;
            movement.d.y = 0.;
        }
        if movement.d.y <= -1. {
            tile.pos.y -= -movement.d.y as u32;
            movement.d.y = 0.;
        }
        if old_pos != tile.pos {
            mov.send(AsciiMoveEvent {
                entity,
                old_pos,
                new_pos: tile.pos
            });
        }
    }
}

pub struct LivingEntityPlugin;
impl Plugin for LivingEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system);
    }
}