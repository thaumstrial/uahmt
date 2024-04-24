use std::fmt::Pointer;
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy_fast_tilemap::{FastTileMapPlugin, Map, MapBundleManaged};
use crate::ascii_world::{AsciiAddEvent, AsciiMoveEvent, AsciiRemoveEvent, AsciiTile};

#[derive(Component)]
struct Layers(Vec<Entity>);

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map>>,
) {
    let tiles_texture = asset_server.load("atlas.png");
    commands.spawn(Camera2dBundle::default());
    let mut layers: Vec<Entity> = Vec::new();
    commands.spawn_empty()
        .with_children(|parent| {
            for z in 0..64u32 {
                let map = Map::builder(
                    uvec2(64, 64),
                    tiles_texture.clone(),
                    vec2(16., 16.),
                ).build();
                let child_id = parent.spawn(MapBundleManaged {
                    material: materials.add(map),
                    transform: Transform::default().with_translation(vec3(0., 0., z as f32)),
                    ..default()
                }).id();
                layers.push(child_id);
            }
        })
        .insert((Layers(layers), InheritedVisibility::VISIBLE, GlobalTransform::default()));
}

fn update_map(
    mut add: EventReader<AsciiAddEvent>,
    mut remove: EventReader<AsciiRemoveEvent>,
    mut mov: EventReader<AsciiMoveEvent>,
    mut materials: ResMut<Assets<Map>>,
    maps: Query<&Handle<Map>>,
    layers: Query<&Layers>
) {

    let layers = layers.single();
    for ev in add.read() {
        let pos = ev.pos;
        let map_handle = maps.get(*layers.0.get(pos.z as usize).unwrap()).unwrap();
        let map = materials.get_mut(map_handle).unwrap();
        let mut m = map.indexer_mut();
        m.set(pos.x, pos.y, '@' as u32);
    }
    for ev in remove.read() {

    }
    for ev in mov.read() {

    }
}

pub struct AsciiRenderPlugin;
impl Plugin for AsciiRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update_map)
            .add_plugins(FastTileMapPlugin::default())
            .insert_resource(ClearColor(Color::BLACK));
    }
}