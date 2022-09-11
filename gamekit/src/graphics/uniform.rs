//
// Shader Uniforms
//

use std::{ffi::CString, marker::PhantomData};

use crate::graphics::{program::Program};

use super::{gl, types::{Vector2i, Vector4, Vector3, Vector2}};

defaults!();

pub struct Uniform<T> {
    name: String,
    location: i32,
    phantom: PhantomData<T>
}

impl<T> Uniform<T> {

    pub fn new(name: &str) -> Uniform<T> {

        let uniform = Uniform {
            name: name.to_string(),
            location: -1,
            phantom: PhantomData
        };

        return uniform;
    }

    fn get_uniform_location(name: &str) -> i32 {

        let program_id = Program::get_current_program();
        if program_id == 0 {
            return -1;
        }

        let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

        let location: i32;

        unsafe {
            location = gl::GetUniformLocation(program_id as gl::types::GLuint, cname.as_bytes_with_nul().as_ptr() as *const i8) as i32;
        }

        return location;

    }

    fn get_location(&mut self) -> i32 {

        if self.location < 0 {
            self.location = Self::get_uniform_location(&self.name);
        }

        return self.location;
    }

}

pub trait Uniform1f {
    fn set(&mut self, value: f32);
}

impl Uniform1f for Uniform<f32> {
    fn set(&mut self, value: f32) {
        let location = self.get_location();
        if location >= 0 {
            unsafe {
                gl::Uniform1f(location as gl::types::GLint, value as gl::types::GLfloat);
            }
        }
    }
}

pub trait Uniform2f {
    fn set(&mut self, value: Vector2);
}

impl Uniform2f for Uniform<Vector2> {
    fn set(&mut self, value: Vector2) {
        let location = self.get_location();
        if location >= 0 {
            unsafe {
                gl::Uniform2f(location as gl::types::GLint,
                              value.x as gl::types::GLfloat,
                              value.y as gl::types::GLfloat);
            }
        }
    }
}

pub trait Uniform3f {
    fn set(&mut self, value: Vector3);
}

impl Uniform3f for Uniform<Vector3> {
    fn set(&mut self, value: Vector3) {
        let location = self.get_location();
        if location >= 0 {
            unsafe {
                gl::Uniform3f(location as gl::types::GLint,
                              value.x as gl::types::GLfloat,
                              value.y as gl::types::GLfloat,
                              value.z as gl::types::GLfloat);
            }
        }
    }
}

pub trait Uniform4f {
    fn set(&mut self, value: Vector4);
}

impl Uniform4f for Uniform<Vector4> {
    fn set(&mut self, value: Vector4) {
        let location = self.get_location();
        if location >= 0 {
            unsafe {
                gl::Uniform4f(location as gl::types::GLint,
                              value.x as gl::types::GLfloat,
                              value.y as gl::types::GLfloat,
                              value.z as gl::types::GLfloat,
                              value.w as gl::types::GLfloat);
            }
        }
    }
}

pub trait Uniform1i {
    fn set(&mut self, value: i32);
}

impl Uniform1i for Uniform<i32> {
    fn set(&mut self, value: i32) {
        let location = self.get_location();
        if location >= 0 {
            unsafe {
                gl::Uniform1i(location as gl::types::GLint,
                              value as gl::types::GLint);
            }
        }
    }
}

pub trait Uniform2i {
    fn set(&mut self, value: Vector2i);
}

impl Uniform2i for Uniform<Vector2i> {
    fn set(&mut self, value: Vector2i) {
        let location = self.get_location();
        if location >= 0 {
            unsafe {
                gl::Uniform2i(location as gl::types::GLint,
                              value.x as gl::types::GLint,
                              value.y as gl::types::GLint);
            }
        }
    }
}
