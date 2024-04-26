use bevy::prelude::*;
use crate::ascii_world::{AsciiMoveEvent, AsciiTile, WorldSettings};

#[derive(Component)]
pub struct Movement {
    pub v: f32,
    pub d: Vec3
}

pub struct LivingEntityPlugin;
impl Plugin for LivingEntityPlugin {
    fn build(&self, app: &mut App) {

    }
}