//
// Vertex
//

use super::gl;

defaults!();

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y }  }
    pub fn zero() -> Self { Self::new(0.0, 0.0) }
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y }  }
    pub fn zero() -> Self { Self::new(0, 0) }
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z }  }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w }  }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) }
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
}


#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self { Self { x, y, w, h } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0, w: 0.0, h: 0.0 } }
    pub fn one() -> Self { Self { x: 0.0, y: 0.0, w: 1.0, h: 1.0 } }
    pub fn set(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self { Self { red, green, blue, alpha } }
    pub fn zero() -> Color { Self::new(0.0, 0.0, 0.0, 0.0) }
    pub fn one() -> Color { Self::new(1.0, 1.0, 1.0, 1.0) }

    pub fn set(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.red = red;
        self.green = green;
        self.blue = blue;
        self.alpha = alpha;
    }

    pub fn set_color(&mut self, color: &Color) {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
        self.alpha = color.alpha;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Quad {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex
}

impl Quad {

    pub const INDICES: [u32; 6] = [ 3, 2, 0, 2, 1, 0 ];

    pub fn new(coords: Rectangle, color: Color, texture_coords: Rectangle) -> Self {

        let x0 = coords.x;
        let y0 = coords.y;
        let x1 = x0 + coords.w;
        let y1 = y0 + coords.h;
        let z = 0.0f32;

        let u0 = texture_coords.x;
        let v0 = texture_coords.y;
        let u1 = u0 + texture_coords.w;
        let v1 = v0 + texture_coords.h;

        let quad = Quad {
            v0: Vertex::new( x0, y0, z, color, u0, v0 ),
            v1: Vertex::new( x1, y0, z, color, u1, v0 ),
            v2: Vertex::new( x1, y1, z, color, u1, v1 ),
            v3: Vertex::new( x0, y1, z, color, u0, v1 )
        };

        return quad;
    }

    pub fn zero() -> Self {
        Self {
            v0: Vertex::zero(),
            v1: Vertex::zero(),
            v2: Vertex::zero(),
            v3: Vertex::zero()
        }
    }

    pub fn one() -> Self {
        Self {
            v0: Vertex::new(0.0, 0.0, 0.0, Color::one(), 0.0, 0.0),
            v1: Vertex::new(1.0, 0.0, 0.0, Color::one(), 1.0, 0.0),
            v2: Vertex::new(1.0, 1.0, 0.0, Color::one(), 1.0, 1.0),
            v3: Vertex::new(0.0, 1.0, 0.0, Color::one(), 0.0, 1.0)
        }
    }

    pub fn to_vector(self) -> Vec<Vertex> {
        vec! [
            self.v0,
            self.v1,
            self.v2,
            self.v3
        ]
    }

}
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    //#[location = "0"]
    pub coords: Vector3,
    //#[location = "1"]
    pub color: Color,
    //#[location = "2"]
    pub texcoords: Vector2
}

impl Vertex {

    pub fn new(x: f32, y: f32, z: f32, color: Color, u: f32, v: f32) -> Self {
        let coords = Vector3::new(x, y, z);
        let color = color;
        let texcoords = Vector2::new(u, v);
        let vertex = Vertex { coords, color, texcoords };
        return vertex;
    }

    pub fn zero() -> Self {
        Self {
            coords: Vector3::new(0.0, 0.0, 0.0),
            color: Color::zero(),
            texcoords: Vector2::new(0.0, 0.0)
        }
    }

    pub fn declare_attrib_pointers(start_index: u32) {

        let mut index = start_index as gl::types::GLuint;
        let mut offset = 0 as gl::types::GLuint;
        let stride = std::mem::size_of::<Vertex>() as gl::types::GLsizei;
        let float_size = std::mem::size_of::<f32>() as u32;

        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(index, 3, gl::FLOAT, gl::FALSE, stride, offset as *const gl::types::GLvoid);
            index += 1; offset += 3 * float_size;

            gl::VertexAttribPointer(index, 4, gl::FLOAT, gl::FALSE, stride, offset as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(index);
            index += 1; offset += 4 * float_size;

            gl::VertexAttribPointer(index, 2, gl::FLOAT, gl::FALSE, stride, offset as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(index);
            //index += 1; offset += 2 * float_size;
        };

    }

    pub fn set_color(&mut self, color: &Color) {
        self.color.set_color(color);
    }


}
