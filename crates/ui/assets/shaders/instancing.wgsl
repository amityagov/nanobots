#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    var out: VertexOutput;
    // NOTE: Passing 0 as the instance_index to get_world_from_local() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(position, 1.0)
    );
    out.color = vertex.i_color;
    out.normal = vertex.normal;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.normal);
    let light_direction = vec3<f32>(0.0, 1.0, 0.0); // Light from top of scene

    // Compute the dot product between the normal and light direction
    let diffuse = max(dot(normal, light_direction), 0.0);

    // Apply diffuse lighting to the color
    // let base_color = vec3<f32>(0.2, 0.4, 0.8); // Nice blue color
    let base_color = in.color.rgb;
    let shaded_color = base_color * diffuse;

    // Add ambient lighting
    let ambient_strength = 0.2;
    let ambient_color = base_color * ambient_strength;
    
    // Combine ambient and diffuse lighting
    let final_color = ambient_color + shaded_color;

    // Output the final color with full alpha
    return vec4<f32>(final_color, 1.0);
}
