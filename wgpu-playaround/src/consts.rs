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

pub const STAR_VERTICES: &[Vertex] = &[
    // Outer points (tips of the star) - radius 0.5
    // Angles: 90°, 18°, -54°, -126°, 162°
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },           // 0 - Top (90°)
    Vertex { position: [0.475528, 0.154509, 0.0], color: [0.0, 1.0, 0.0] }, // 1 - Right-upper (18°)
    Vertex { position: [0.293893, -0.404509, 0.0], color: [0.0, 0.0, 1.0] }, // 2 - Right-lower (-54°)
    Vertex { position: [-0.293893, -0.404509, 0.0], color: [1.0, 0.0, 1.0] }, // 3 - Left-lower (-126°)
    Vertex { position: [-0.475528, 0.154509, 0.0], color: [0.0, 1.0, 1.0] }, // 4 - Left-upper (162°)

    // Inner points (valleys between tips) - radius 0.2
    // Angles: 54°, -18°, -90°, -162°, 126°
    Vertex { position: [0.161803, 0.117557, 0.0], color: [1.0, 1.0, 0.0] },  // 5 - Between 0 and 1 (54°)
    Vertex { position: [0.061803, -0.190211, 0.0], color: [1.0, 0.5, 0.0] }, // 6 - Between 1 and 2 (-18°)
    Vertex { position: [0.0, -0.2, 0.0], color: [0.5, 1.0, 0.0] },           // 7 - Between 2 and 3 (-90°)
    Vertex { position: [-0.061803, -0.190211, 0.0], color: [0.0, 1.0, 0.5] }, // 8 - Between 3 and 4 (-162°)
    Vertex { position: [-0.161803, 0.117557, 0.0], color: [0.5, 0.0, 1.0] }, // 9 - Between 4 and 0 (126°)

    // Center point
    Vertex { position: [0.0, 0.0, 0.0], color: [1.0, 1.0, 1.0] },           // 10 - Center
];

// Create triangles for a proper 5-pointed star
// Each tip forms a triangle with its two adjacent inner points
// Each valley forms a triangle with the center and its two adjacent inner points
pub const STAR_INDICES: &[u16] = &[
    // Outer triangles (tips of the star)
    0, 9, 5,   // Top tip (0) with inner points 9 and 5
    1, 5, 6,   // Right-upper tip (1) with inner points 5 and 6
    2, 6, 7,   // Right-lower tip (2) with inner points 6 and 7
    3, 7, 8,   // Left-lower tip (3) with inner points 7 and 8
    4, 8, 9,   // Left-upper tip (4) with inner points 8 and 9
    
    // Inner triangles (valleys, connecting to center)
    10, 9, 5,  // Center to valley between tip 0 and 1
    10, 5, 6,  // Center to valley between tip 1 and 2
    10, 6, 7,  // Center to valley between tip 2 and 3
    10, 7, 8,  // Center to valley between tip 3 and 4
    10, 8, 9,  // Center to valley between tip 4 and 0
];

