#import bevy_sprite::mesh2d_bindings::mesh
#import bevy_sprite::mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip, mesh2d_position_local_to_world}

@group(2) @binding(0) var<uniform> tile_size: vec2<u32>;
@group(2) @binding(1) var<uniform> atlas_size: vec2<u32>;
@group(2) @binding(2) var<uniform> texture_index: u32;
@group(2) @binding(3) var<uniform> ft_color: vec4<f32>;
@group(2) @binding(4) var<uniform> bg_color: vec4<f32>;
@group(2) @binding(10) var atlas: texture_2d<f32>;
@group(2) @binding(11) var atlas_sampler: sampler;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>
}
@vertex
fn vertex(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    var model: mat4x4<f32> = get_model_matrix(in.instance_index);
    out.position = mesh2d_position_local_to_clip(model, vec4<f32>(in.position, 1.0));
    return out;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    var atlas_x = f32(atlas_size.x);
    var atlas_y = f32(atlas_size.y);
    var index_i = f32(texture_index);
    var tile_x = f32(tile_size.x);
    var index_y = floor(index_i * tile_x / atlas_x);
    var index_x = index_i * tile_x - index_y * atlas_x;
    var texture_start = vec2<f32>(index_x, index_y * atlas_y);
    // 根据texture_start采样texture
    var color = textureSample(atlas, atlas_sampler, texture_start / atlas_size, 0);



    return color;
}