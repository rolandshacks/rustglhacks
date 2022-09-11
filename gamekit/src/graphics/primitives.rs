use super::{gl};

extern crate sdl2;

defaults!();

#[allow(dead_code)]
pub enum DrawMode {
    Triangles,
    TriangleStrip
}


pub struct Primitives {}

impl Primitives {

    pub fn viewport(x: u32, y: u32, width: u32, height: u32) {

        unsafe {
            gl::Viewport(x as gl::types::GLsizei,
                         y as gl::types::GLsizei,
                         width as gl::types::GLsizei,
                         height as gl::types::GLsizei);
        }
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw_arrays(mode: DrawMode, ofs: usize, count: usize) {

        let draw_mode:  gl::types::GLenum;

        match mode {
            DrawMode::Triangles => { draw_mode = gl::TRIANGLES; },
            DrawMode::TriangleStrip => { draw_mode = gl::TRIANGLE_STRIP; },
        }

        unsafe {
            gl::DrawArrays(
                draw_mode,
                ofs as gl::types::GLint,     // starting index in the enabled arrays
                count as gl::types::GLsizei         // number of indices to be rendered
            );
        }
    }

    pub fn draw_elements(mode: DrawMode, count: usize) {

        let draw_mode:  gl::types::GLenum;

        match mode {
            DrawMode::Triangles => { draw_mode = gl::TRIANGLES; },
            DrawMode::TriangleStrip => { draw_mode = gl::TRIANGLE_STRIP; },
        }

        unsafe {
            gl::DrawElements(
                draw_mode,
                count as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                0 as *const _
            );
        }
    }

    pub fn draw_elements_instanced(mode: DrawMode, count: usize, num_instances: usize) {

        let draw_mode:  gl::types::GLenum;

        match mode {
            DrawMode::Triangles => { draw_mode = gl::TRIANGLES; },
            DrawMode::TriangleStrip => { draw_mode = gl::TRIANGLE_STRIP; },
        }

        unsafe {
            gl::DrawElementsInstanced(
                draw_mode,
                count as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                0 as *const _,
                num_instances as gl::types::GLsizei
            );
        }

    }

    pub fn bind_texture(id: u32, bind_location: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + bind_location);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }
    }

    pub fn unbind_texture(_id: u32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

}
