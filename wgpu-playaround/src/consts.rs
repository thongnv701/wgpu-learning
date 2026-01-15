use crate::models::vertex::Vertex;

/* pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0]},
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0]},
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
]; */

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

// Star shape (8 triangles)
pub const STAR_VERTICES: &[Vertex] = &[
    // Center
    Vertex { position: [0.0, 0.0, 0.0], color: [1.0, 1.0, 0.0] }, // 0 - Center (yellow)
    // Outer points
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },     // 1 - Top (red)
    Vertex { position: [0.35, 0.35, 0.0], color: [0.0, 1.0, 0.0] },   // 2 (green)
    Vertex { position: [0.5, 0.0, 0.0], color: [0.0, 0.0, 1.0] },     // 3 - Right (blue)
    Vertex { position: [0.35, -0.35, 0.0], color: [1.0, 0.0, 1.0] },  // 4 (magenta)
    Vertex { position: [0.0, -0.5, 0.0], color: [0.0, 1.0, 1.0] },    // 5 - Bottom (cyan)
    Vertex { position: [-0.35, -0.35, 0.0], color: [1.0, 1.0, 0.0] }, // 6 (yellow)
    Vertex { position: [-0.5, 0.0, 0.0], color: [1.0, 0.0, 0.0] },    // 7 - Left (red)
    Vertex { position: [-0.35, 0.35, 0.0], color: [0.0, 1.0, 0.0] },  // 8 (green)
];

pub const STAR_INDICES: &[u16] = &[
    0, 1, 2, // Triangle 1
    0, 2, 3, // Triangle 2
    0, 3, 4, // Triangle 3
    0, 4, 5, // Triangle 4
    0, 5, 6, // Triangle 5
    0, 6, 7, // Triangle 6
    0, 7, 8, // Triangle 7
    0, 8, 1, // Triangle 8
];
