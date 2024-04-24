#import bevy_sprite::mesh2d_bindings::mesh
#import bevy_sprite::mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip, mesh2d_position_local_to_world}

@group(2) @binding(0) var<uniform> tile_size: vec2<u32>;
@group(2) @binding(1) var<uniform> ascii: u32;
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
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}