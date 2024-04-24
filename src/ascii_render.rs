use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayout};
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat};
use bevy::sprite::{Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};
use crate::ascii_world::AsciiTile;

fn startup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct AsciiRenderSettings {
    tile_size: UVec2,
    atlas: Handle<Image>,
    mesh: Mesh2dHandle,
}
impl FromWorld for AsciiRenderSettings {
    fn from_world(world: &mut World) -> Self {
        let atlas = world.get_resource::<AssetServer>().unwrap().load("atlas.png");
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let tile_size = UVec2::new(16, 16);
        let mesh = Mesh2dHandle(meshes.add(
            Mesh::from(Rectangle {
                half_size: (tile_size / 2).as_vec2()
            }))
        );

        Self {
            tile_size,
            atlas,
            mesh
        }
    }
}

#[derive(Asset, TypePath, Debug, Clone, Default, AsBindGroup)]
struct AsciiMaterial {
    #[uniform(0)]
    tile_size: UVec2,
    #[uniform(1)]
    ascii: u32,
    #[texture(10)]
    #[sampler(11)]
    atlas: Handle<Image>,
}
impl Material2d for AsciiMaterial {
    fn vertex_shader() -> ShaderRef {
        "ascii.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "ascii.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: Material2dKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

fn add_shader(
    mut commands: Commands,
    q: Query<(Entity, &AsciiTile), (Without<Mesh2dHandle>, Without<Handle<AsciiMaterial>>)>,
    settings: Res<AsciiRenderSettings>,
    mut materials: ResMut<Assets<AsciiMaterial>>
) {
    for (entity, tile) in q.iter() {
        let mesh = settings.mesh.clone();
        let material = materials.add(AsciiMaterial {
            tile_size: settings.tile_size,
            ascii: '@' as u32,
            atlas: settings.atlas.clone(),
        });
        let transform = Transform::from_translation(tile.pos.as_vec3());
        commands.entity(entity).insert(MaterialMesh2dBundle {
                mesh,
                transform,
                material,
                ..default()
            });
    }
}

pub struct AsciiRenderPlugin;
impl Plugin for AsciiRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<AsciiMaterial>::default())
            .add_systems(Startup, startup)
            .add_systems(Update, add_shader)
            .init_resource::<AsciiRenderSettings>()
            .insert_resource(ClearColor(Color::BLACK));
    }
}