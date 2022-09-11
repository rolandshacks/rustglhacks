//
// Std140
//

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct vec2 {
    pub x: f32,
    pub y: f32,
    _padding0: f32,
    _padding1: f32
}

impl vec2 {
    pub fn new(x: f32, y: f32) -> vec2 {
        vec2 { x, y, _padding0: 0.0, _padding1: 0.0 }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _padding0: f32,
}

impl vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> vec3 {
        vec3 { x, y, z, _padding0: 0.0 }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> vec4 {
        vec4 { x, y, z, w }
    }
}
