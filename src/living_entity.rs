use bevy::prelude::*;
use crate::ascii_world::{AsciiMoveEvent, AsciiTile, WorldSettings};

#[derive(Component)]
pub struct Movement {
    pub v: f32,
    pub d: Vec3
}

pub fn movement_system(
    mut q: Query<(Entity, &mut Movement, &mut AsciiTile)>,
    mut mov: EventWriter<AsciiMoveEvent>,
    settings: Res<WorldSettings>
) {
    for (entity, mut movement, mut tile) in q.iter_mut() {
        let old_pos = tile.pos.clone();
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
        if old_pos != new_pos {
            tile.pos = new_pos;
            mov.send(AsciiMoveEvent {
                entity,
                old_pos,
                new_pos
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