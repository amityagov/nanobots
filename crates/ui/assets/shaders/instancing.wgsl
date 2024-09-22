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
    // Calculate the distance from the fragment to the nearest edge
    let edge_distance = min(min(in.color.x, in.color.y), in.color.z);
    
    // Define edge thickness
    let edge_thickness = 0.05;
    
    // Create an edge color that's different from the vertex color
    let edge_color = vec4<f32>(1.0 - in.color.rgb, in.color.a);
    
    // Interpolate between edge color and vertex color based on edge distance
    let final_color = mix(edge_color, in.color, smoothstep(0.0, edge_thickness, edge_distance));
    // Define a light direction (you might want to pass this as a uniform in a real scenario)
    let light_direction = normalize(vec3<f32>(1.0, 1.0, 1.0));

    let normal = normalize(in.normal);

    // Calculate the diffuse factor
    let diffuse_factor = max(dot(normal, light_direction), 0.0);

    // Add ambient light
    let ambient_strength = 0.1;
    let ambient = ambient_strength * final_color.rgb;

    // Calculate final color with lighting
    let lit_color = ambient + diffuse_factor * final_color.rgb;

    return vec4<f32>(lit_color, final_color.a);
    // return vec4<f32>(0.1, 0.4, 0.8, 1.0); // Nice blue color
}