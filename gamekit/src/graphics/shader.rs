//
// Shader
//

defaults!();

use std::ffi::c_void;
use std::path::Path;
use std::{ffi::{CString}, fs};

use crate::graphics::gl;

#[derive(Clone, Copy)]
pub enum ShaderType {
    VertexShader,
    FragmentShader
}

pub struct Shader {
    id: u32,
    _shader_type: ShaderType,
    _filename: String
}

impl Shader {

    pub fn new(shader_type: ShaderType, filename: &str) -> Result<Shader, String> {
        return load_shader(shader_type, filename);
    }

    #[allow(dead_code)]
    pub fn shader_type(&self) -> ShaderType {
        return self._shader_type;
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

    pub fn free(&mut self) {
        info!("Shader free");
        if self.id != 0 {
            unsafe {
                gl::DeleteShader(self.id);
            }
            self.id = 0;
        }
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        debug!("drop shader");
        self.free();
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn load_shader(shader_type: ShaderType, filename: &str) -> Result<Shader, String> {

    let kind: gl::types::GLuint;

    match shader_type {
        ShaderType::VertexShader => { kind = gl::VERTEX_SHADER; },
        ShaderType::FragmentShader => { kind = gl::FRAGMENT_SHADER; }
    }

    let file_ext = Path::new(&filename).extension();
    if file_ext.is_none() {
        return Err(format!("failed to access shader file {}", filename));
    }

    let file_ext = file_ext.unwrap();

    let is_binary = if file_ext.eq_ignore_ascii_case("spv") { true } else { false };

    let id = unsafe { gl::CreateShader(kind) };

    if is_binary {

        let shader_binary = fs::read(filename);
        let mut shader_binary = match shader_binary {
            Ok(binary) => binary,
            Err(_) => {
                unsafe { gl::DeleteShader(id); }
                return Err(format!("failed to read shader file {}", filename));
            }
        };

        let shader_binary_size = shader_binary.len();

        let entry_point = &CString::new("main").unwrap();
        let constant_index = 0;
        let constant_value = 0;

        unsafe {
            gl::ShaderBinary(1, &id, gl::SHADER_BINARY_FORMAT_SPIR_V, shader_binary.as_mut_ptr() as *mut c_void, shader_binary_size as gl::types::GLsizei);
            gl::SpecializeShader(id, entry_point.as_ptr(), 0, &constant_index, &constant_value);
        }

    } else {

        let shader_source =fs::read_to_string(filename);
        let shader_source = match shader_source {
            Ok(source) => source,
            Err(_) => {
                unsafe { gl::DeleteShader(id); }
                return Err(format!("failed to read shader file {}", filename));
            }
        };

        let source = &CString::new(shader_source).unwrap();

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

    }

    let mut success: gl::types::GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );

            gl::DeleteShader(id);
        }

        error!("failed to compile shader: {}", error.to_string_lossy());

        return Err(error.to_string_lossy().into_owned());
    }

    let shader = Shader {
        id,
        _shader_type: shader_type,
        _filename: filename.to_string()
    };

    return Ok(shader);

}
