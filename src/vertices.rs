#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}


impl Vec3 {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pos: Vec3,
    tex_coord: Vec2,
    normal: Vec3,
}


pub static VERTICES: &[Vertex] = &[
    // First face (PZ)
    // First triangle
    Vertex {
        pos: Vec3::new(-0.5, -0.5, 0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, 0.5),
        tex_coord: Vec2::new(1.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, 0.5),
        tex_coord: Vec2::new(0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, 0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    },

    // Second face (MZ)
    // First triangle
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, -0.5),
        tex_coord: Vec2::new(1.0, 0.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, -0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, -0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
    },

    // Third face (PX)
    // First triangle
    Vertex {
        pos: Vec3::new(0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, -0.5),
        tex_coord: Vec2::new(1.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, 0.5),
        tex_coord: Vec2::new(0.0, 1.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    },

    // Fourth face (MX)
    // First triangle
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, 0.5),
        tex_coord: Vec2::new(1.0, 0.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, -0.5),
        tex_coord: Vec2::new(0.0, 1.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
    },

    // Fifth face (PY)
    // First triangle
    Vertex {
        pos: Vec3::new(-0.5, 0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, -0.5),
        tex_coord: Vec2::new(0.0, 1.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    },

    // Sixth face (MY)
    // First triangle
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, -0.5),
        tex_coord: Vec2::new(1.0, 0.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, 0.5),
        tex_coord: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, 0.5),
        tex_coord: Vec2::new(0.0, 1.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -0.5),
        tex_coord: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    },
];