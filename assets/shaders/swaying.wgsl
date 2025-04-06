#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::view,
}

@group(2) @binding(0)
var<uniform> sway_strength: f32;
@group(2) @binding(1)
var<uniform> sway_speed: f32;
@group(2) @binding(2)
var<uniform> vertical_influence: f32;
@group(2) @binding(3)
var texture: texture_2d<f32>;
@group(2) @binding(4)
var texture_sampler: sampler;
@group(2) @binding(5)
var<uniform> time: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let height_factor = (1 - vertex.uv.y) * vertical_influence;
    let sway = sin(time * sway_speed + vertex.position.y * 0.5) * 10; // sway_strength;
    let adjusted_position = vertex.position - vec3<f32>(sway * height_factor, 0.0, 0.0);

    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);

    out.position = mesh_functions::mesh2d_position_local_to_clip(world_from_local, vec4<f32>(adjusted_position, 1.0));
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        world_from_local,
        vec4<f32>(adjusted_position, 1.0)
    );
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = textureSample(texture, texture_sampler, in.uv);
    return color;
}