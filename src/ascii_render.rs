use std::fmt::Pointer;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::utils::tracing::Instrument;
use bevy_fast_tilemap::{CustomFastTileMapPlugin, FastTileMapPlugin, Map, MapBundleManaged};
use crate::ascii_world::{AsciiAddEvent, AsciiMoveEvent, AsciiRemoveEvent, AsciiTile, WorldSettings};
use crate::player::PlayerMarker;

#[derive(Component)]
struct Layers(Vec<Entity>);
#[derive(Component)]
pub struct ViewLayer(pub u32);
#[derive(Event)]
pub struct UpdateViewLayerEvent(pub u32);

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map<UserData>>>,
    mut settings: Res<WorldSettings>,
    player_query: Query<&AsciiTile, With<PlayerMarker>>
) {
    let tiles_texture = asset_server.load("atlas.png");
    if let Ok(tile) =  player_query.get_single() {
        commands.spawn(Camera2dBundle::default()).insert(ViewLayer(tile.pos.z));
    }
    let mut layers: Vec<Entity> = Vec::new();
    commands.spawn_empty()
        .with_children(|parent| {
            for z in 0..settings.size.z {
                let map = Map::<UserData>::builder(
                    settings.size.xy(),
                    tiles_texture.clone(),
                    vec2(16., 16.),
                )
                    .with_user_data(UserData { alpha: 1.})
                    .build_and_initialize(
                        |m| {
                            for y in 0..m.size().y {
                                for x in 0..m.size().y {
                                    m.set(x, y, ' ' as u32, Color::NONE, Color::NONE);
                                }
                            }
                        }
                    );
                let child_id = parent.spawn(MapBundleManaged::<UserData> {
                    material: materials.add(map),
                    transform: Transform::default().with_translation(vec3(0., 0., z as f32)),
                    ..default()
                }).id();
                layers.push(child_id);
            }
        })
        .insert((Layers(layers), InheritedVisibility::HIDDEN, GlobalTransform::default()));
}

fn add_event_reader(
    mut add: EventReader<AsciiAddEvent>,
    mut mov: EventWriter<AsciiMoveEvent>,
    mut materials: ResMut<Assets<Map<UserData>>>,
    maps: Query<&Handle<Map<UserData>>>,
    layers: Query<&Layers>
) {

    let layers = layers.single();
    for ev in add.read() {
        let pos = ev.pos;
        let map_handle = maps.get(*layers.0.get(pos.z as usize).unwrap()).unwrap();
        let map = materials.get_mut(map_handle).unwrap();
        let mut m = map.indexer_mut();
        m.set(pos.x, pos.y, '@' as u32, Color::PINK, Color::NONE);
    }
}
fn move_event_reader(
    mut mov: EventReader<AsciiMoveEvent>,
    mut materials: ResMut<Assets<Map<UserData>>>,
    maps: Query<&Handle<Map<UserData>>>,
    layers: Query<&Layers>
) {

    let layers = layers.single();
    for ev in mov.read() {
        {
            let old_pos = ev.old_pos;
            let map_handle = maps.get(*layers.0.get(old_pos.z as usize).unwrap()).unwrap();
            let map = materials.get_mut(map_handle).unwrap();
            let mut m = map.indexer_mut();
            m.set(old_pos.x, old_pos.y, ' ' as u32, Color::NONE, Color::NONE);
        }
        {
            let new_pos = ev.new_pos;
            let map_handle = maps.get(*layers.0.get(new_pos.z as usize).unwrap()).unwrap();
            let map = materials.get_mut(map_handle).unwrap();
            let mut m = map.indexer_mut();
            m.set(new_pos.x, new_pos.y, '@' as u32, Color::PINK, Color::NONE);
        }
    }
}

fn camera_control(
    key: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    settings: Res<WorldSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(
        &GlobalTransform,
        &mut Transform,
        &Camera,
        &mut OrthographicProjection,
        &mut ViewLayer
    )>,
    mut update_view_layer: EventWriter<UpdateViewLayerEvent>,
) {
    if key.pressed(KeyCode::ControlLeft) {
        for event in mouse_motion_events.read() {
            if mouse_button.pressed(MouseButton::Left) || mouse_button.pressed(MouseButton::Right) {
                for (_, mut transform, _, _, _) in camera_query.iter_mut() {
                    transform.translation.x -= event.delta.x * transform.scale.x;
                    transform.translation.y += event.delta.y * transform.scale.y;
                }
            }
        }
        let mut wheel_y = 0.;
        for event in mouse_wheel_events.read() {
            wheel_y += event.y;
        }
        if wheel_y != 0. {
            for (_, mut transform, _, mut _ortho, _) in camera_query.iter_mut() {
                let factor = f32::powf(2., -wheel_y / 2.);
                transform.scale *= vec3(factor, factor, 1.0);
                transform.scale = transform
                    .scale
                    .max(Vec3::splat(1. / 128.))
                    .min(Vec3::splat(128.));
            }
        }
    } else {
        for (_,  _, _,  _, mut view) in camera_query.iter_mut() {
            let mut wheel_y = 0.;
            for event in mouse_wheel_events.read() {
                wheel_y += event.y;
            }
            wheel_y = wheel_y.floor();
            if wheel_y >= 1. && view.0 < settings.size.z - 1 {
                view.0 += 1;
            } else if wheel_y <= -1. && view.0 > 0{
                view.0 -= 1;
            }
            update_view_layer.send(UpdateViewLayerEvent(view.0));
        }
    }
}

fn update_visibility(
    settings: Res<WorldSettings>,
    mut commands: Commands,
    mut update: EventReader<UpdateViewLayerEvent>,
    mut materials: ResMut<Assets<Map<UserData>>>,
    maps: Query<&Handle<Map<UserData>>>,
    layers: Query<&Layers>
) {
    if let Ok(layers) = layers.get_single() {
        for ev in update.read() {
            for i in  0..settings.size.z {
                let map_handle = maps.get(*layers.0.get(i as usize).unwrap()).unwrap();
                let mut map = materials.get_mut(map_handle).unwrap();
                let mut layer = commands.entity(*layers.0.get(i as usize).unwrap());
                if i <= ev.0 {
                    map.user_data.alpha = (i + 5) as f32 / (ev.0 + 5) as f32 ;
                } else {
                    map.user_data.alpha = 0.0;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Reflect, AsBindGroup, ShaderType)]
struct UserData {
    alpha: f32
}


pub struct AsciiRenderPlugin;
impl Plugin for AsciiRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UpdateViewLayerEvent>()
            .add_systems(PostStartup, startup)
            .add_systems(Update, (
                add_event_reader,
                move_event_reader
                ))
            .add_systems(Update, camera_control)
            .add_systems(Update, update_visibility)
            .add_plugins(CustomFastTileMapPlugin::<UserData> {
                user_code: Some(
                    r#"
                    struct UserData {
                        alpha: f32
                    };
                    fn sample_tile(in: ExtractIn) -> vec4<f32> {
                        var tile_index = in.tile_index;
                        var tile_position = in.tile_position;
                        var tile_offset = in.tile_offset;
                        var tile_ft_color = get_tile_ft_color(tile_position);
                        var tile_bg_color = get_tile_bg_color(tile_position);

                        var tile_start = atlas_index_to_position(tile_index);
                        // Offset in pixels from tile_start to sample from
                        var rect_offset = floor(tile_offset) + map.tile_anchor_point * map.tile_size;
                        var total_offset = tile_start + rect_offset;

                        // At most half of the inner "padding" is still rendered
                        // as overhang of any given tile.
                        // Outer padding is not taken into account
                        var max_overhang = map.inner_padding / 2.0;

                        // Outside of "our" part of the padding, dont render anything as part of this tile,
                        // as it might be used for overhang of a neighbouring tile in the tilemap
                        if rect_offset.x < -max_overhang.x
                            || rect_offset.y < -max_overhang.y
                            || rect_offset.x >= (map.tile_size.x + max_overhang.x)
                            || rect_offset.y >= (map.tile_size.y + max_overhang.y)
                        {
                            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
                        }

                        var color = textureSample(
                            atlas_texture, atlas_sampler, total_offset / map.atlas_size
                        );
                        if color.a == 0.0 {
                            color = tile_bg_color;
                        } else {
                            color *= tile_ft_color;
                        }
                        color *= vec4<f32>(1.0, 1.0, 1.0, user_data.alpha);
                        return color;
                    }
                    "#.to_string(),
                ),
                ..default()
            })
            .insert_resource(ClearColor(Color::BLACK));
    }
}