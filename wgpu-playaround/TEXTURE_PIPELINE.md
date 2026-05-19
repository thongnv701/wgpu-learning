# How to Display an Image (Texture) on a Shape in wgpu 28

This document explains the full pipeline required to load a PNG image and display it on a pentagon in wgpu 28.

---

## Overview

Displaying a texture on a shape requires work across **4 files**:

| File | What you do there |
|---|---|
| `texture.rs` | Load image, create GPU texture, view, sampler |
| `src/models/vertex.rs` | Add `tex_coords` field to `Vertex` struct |
| `src/consts.rs` | Set UV coordinate values for each pentagon vertex |
| `src/shader.wgsl` | Pass UV coords through vertex shader, sample texture in fragment shader |
| `src/models/state.rs` | Create bind group layout, bind group, pipeline layout, call `set_bind_group` each frame |

---

## Step 1 — Add the image file

Place your image in the `asset/` folder at the repo root:

```
wgpu-playaround/
  asset/
    happy_tree.png    ← here
  src/
    ...
```

---

## Step 2 — Create `texture.rs`

This module handles everything related to loading and preparing a texture for the GPU.

```rust
// src/texture.rs
use image::GenericImageView;
use anyhow::*;

pub struct Texture {
    pub texture: wgpu::Texture,   // the raw GPU texture object
    pub view: wgpu::TextureView,  // how the GPU reads the texture
    pub sampler: wgpu::Sampler,   // how the GPU filters/wraps the texture
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions(); // (width, height) in pixels

        // Describe the texture size
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        // Create an empty GPU texture slot
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Upload the pixel data from CPU to the GPU texture
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0), // 4 bytes per pixel (RGBA)
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // Create a view (how to read the texture in shaders)
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a sampler (how to filter pixels when scaling up/down)
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self { texture, view, sampler })
    }
}
```

---

## Step 3 — Add `tex_coords` to the Vertex struct

The GPU needs UV coordinates per vertex so it knows which part of the texture maps to each corner.

```rust
// src/models/vertex.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position:   [f32; 3],  // location in 3D space
    pub color:      [f32; 3],  // vertex color (used in colored pipeline)
    pub tex_coords: [f32; 2],  // UV coordinates → which pixel of the texture
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS, // ← must reference ATTRIBS, not an inline array
        }
    }
}
```

> **Important**: `desc()` must use `&Self::ATTRIBS`. If you write an inline array `&[...]` inside `desc()` that only has 2 entries, the GPU will panic at runtime saying "Location[2] not provided" even though ATTRIBS has 3.

---

## Step 4 — Set UV coordinates for each pentagon vertex

UV space: `(0.0, 0.0)` = top-left of image, `(1.0, 1.0)` = bottom-right.

Formula from NDC (normalized device coordinates) to UV:
```
tex_x = (pos_x + 1.0) / 2.0
tex_y = 1.0 - (pos_y + 1.0) / 2.0   ← Y is flipped (screen Y vs texture Y)
```

```rust
// src/consts.rs
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241,  0.49240386, 0.0], color: [0.5, 0.0, 0.5], tex_coords: [0.4131759,  0.00759614] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5], tex_coords: [0.0048659444, 0.43041354] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5], tex_coords: [0.28081453,  0.949397]  }, // C
    Vertex { position: [0.35966998,  -0.3473291, 0.0], color: [0.5, 0.0, 0.5], tex_coords: [0.85967,     0.84732914] }, // D
    Vertex { position: [0.44147372,  0.2347359,  0.0], color: [0.5, 0.0, 0.5], tex_coords: [0.85967,     0.1526709]  }, // E
];
```

---

## Step 5 — Update the WGSL shader

Two things must happen in the shader:
1. Declare the texture and sampler bindings
2. Pass `tex_coords` through the vertex shader to the fragment shader

```wgsl
// src/shader.wgsl

struct VertexInput {
    @location(0) position:   vec3<f32>,
    @location(1) color:      vec3<f32>,
    @location(2) tex_coords: vec2<f32>,  // ← added
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color:      vec3<f32>,
    @location(1) tex_coords: vec2<f32>,  // ← added
};

// Texture and sampler bound from Rust side at group 0, binding 0 and 1
@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture at the interpolated UV coordinate
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color      = model.color;
    out.tex_coords = model.tex_coords;  // ← must pass through
    return out;
}

@vertex
fn vs_solid(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color      = vec3<f32>(1.0, 0.0, 0.0);
    out.tex_coords = model.tex_coords;  // ← must pass through, even in solid mode
    return out;
}
```

> **Common mistake**: If `vs_solid` (or any vertex shader) does NOT set `out.tex_coords`, every pixel will sample the texture at `(0.0, 0.0)` — a single pixel color repeated across the entire shape.

---

## Step 6 — Wire everything together in `state.rs`

### 6a. Load the texture

```rust
let diffuse_bytes = include_bytes!("../../asset/happy_tree.png");
// Path is relative to the .rs source file, not the project root
// state.rs is at src/models/state.rs → go up 2 levels to reach asset/

let diffuse_texture = texture::Texture::from_bytes(
    &device, &queue, diffuse_bytes, "happy_tree.png"
).unwrap();
// from_bytes does everything: decode PNG, create GPU texture, upload pixels, create view+sampler
```

### 6b. Create a Bind Group Layout (the blueprint)

The layout tells the GPU: "at group 0, binding 0 there is a texture; at binding 1 there is a sampler."

```rust
let texture_bind_group_layout = device.create_bind_group_layout(
    &wgpu::BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    }
);
```

### 6c. Create a Bind Group (the actual data)

```rust
let diffuse_bind_group = device.create_bind_group(
    &wgpu::BindGroupDescriptor {
        label: Some("diffuse_bind_group"),
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
            },
        ],
    }
);
```

### 6d. Include the layout in the pipeline layout

```rust
let render_pipeline_layout = device.create_pipeline_layout(
    &wgpu::PipelineLayoutDescriptor {
        label: Some("Render pipeline layout"),
        bind_group_layouts: &[&texture_bind_group_layout], // ← not &[] or &[Some(...)]
        immediate_size: 0,
    }
);
```

### 6e. Store the bind group in State

```rust
pub struct State {
    // ... other fields ...
    diffuse_bind_group: wgpu::BindGroup,
}
```

### 6f. Set the bind group every frame in `render()`

```rust
render_pass.set_pipeline(pipeline);
render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
// ↑ This must be called every frame, after set_pipeline
```

---

## Full Data Flow Diagram

```
PNG file on disk
    │
    │  include_bytes!("../../asset/happy_tree.png")
    ▼
Raw bytes in CPU memory
    │
    │  image::load_from_memory() → DynamicImage → to_rgba8()
    ▼
RGBA pixel array (width × height × 4 bytes)
    │
    │  device.create_texture()  ← creates empty GPU texture slot
    │  queue.write_texture()    ← uploads pixels CPU → GPU
    ▼
wgpu::Texture (lives on GPU VRAM)
    │
    ├─ texture.create_view()   → wgpu::TextureView  (how shaders read it)
    └─ device.create_sampler() → wgpu::Sampler      (filtering/wrapping rules)
                │
                │  device.create_bind_group_layout()  ← blueprint: "slot 0 = texture, slot 1 = sampler"
                │  device.create_bind_group()         ← actual binding of view + sampler
                ▼
        wgpu::BindGroup
                │
                │  included in pipeline layout
                ▼
        wgpu::RenderPipeline
                │
    ┌───────────┴────────────┐
    │                        │
Vertex Buffer           BindGroup set per frame
(position + color       render_pass.set_bind_group(0, ...)
 + tex_coords)
    │                        │
    │  vertex shader         │
    │  reads tex_coords ─────┘
    │  passes to fragment shader
    ▼
Fragment shader
    │
    │  textureSample(t_diffuse, s_diffuse, in.tex_coords)
    │  ↑ looks up the pixel color at the UV coordinate
    ▼
Screen pixel color
```

---

## UV Coordinate Reference

```
Texture space:
(0,0) ──────── (1,0)
  │                │
  │   happy_tree   │
  │                │
(0,1) ──────── (1,1)

NDC (vertex position) space:
        (0, +1)
           │
(-1, 0) ───┼─── (+1, 0)
           │
        (0, -1)

Conversion:
  tex_x =       (pos_x + 1.0) / 2.0
  tex_y = 1.0 - (pos_y + 1.0) / 2.0   ← Y flipped!
```

---

## Common Errors

| Error | Cause | Fix |
|---|---|---|
| Runtime panic: `Location[2] not provided` | `desc()` uses inline 2-entry array, ignoring `ATTRIBS` | Use `attributes: &Self::ATTRIBS` |
| Solid single color instead of texture | `vs_solid` doesn't set `out.tex_coords` | Add `out.tex_coords = model.tex_coords` to every vertex shader |
| `dimensions` / `texture_size` undefined | Old manual texture code left in state.rs after splitting to texture.rs | Remove the duplicate manual code; use `diffuse_texture.view` and `.sampler` |
| Wrong type for `write_texture` | Passing `&diffuse_texture` (wrapper struct) instead of `&diffuse_texture.texture` (inner wgpu type) | Use the `.texture` field |
| `include_bytes!` path not found | Path is relative to source file, not Cargo.toml | From `src/models/state.rs`, use `../../asset/happy_tree.png` |
| Bind group layout error | Using `&[Some(&layout)]` instead of `&[&layout]` | Remove the `Some()` wrapper |
