struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    // @location(0) vert_pos: vec3<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse:sampler;
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // This will be set by the clear color in render pass
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput{
    var out: VertexOutput;
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);

    return out;
}

/* @vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1-i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1 ) * 0.5;
    out.clip_position = vec4<f32>(x,y,0.0, 1.0);
    // out.vert_pos = out.clip_position.xyz;

    // Generate color based on vertex position
    // Convert from [-0.5, 0.5 ] to [0.0, 1.0] range
    out.color = vec3<f32> (
        (x+0.5),
        (y+0.5),
        0.5
    );

    return out;
} */

@vertex
fn vs_solid(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = vec3<f32>(1.0, 0.0, 0.0);
    out.tex_coords = model.tex_coords;
    return out;
}

/* @vertex
fn vs_colored(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    // Generate color based on vertex position
    out.color = vec3<f32>(
        (x + 0.5),
        (y + 0.5),
        0.5
    );

    return out;
} */

@vertex
fn vs_colored(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);

    out.color = vec3<f32>(
        (model.position.x + 0.5),
        (model.position.y + 0.5),
        0.5
    );

    return out;
}
