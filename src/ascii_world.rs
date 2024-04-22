use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy::utils::HashSet;

#[derive(Deref, Resource)]
pub struct FontHandle(Handle<Font>);
impl FromWorld for FontHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("font.ttf"))
    }
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();


    for x in 0..32u32 {
        for y in 0..32u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex {0: '1' as u32},
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    let texture_handle = asset_server.load("texture.png");
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        }
    ));
}

pub struct AsciiWorldPlugin;
impl Plugin for AsciiWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkManager::default())
            .init_resource::<FontHandle>()
            .add_systems(Startup, startup);
    }
}