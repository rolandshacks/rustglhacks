//
// Program
//

use std::ffi::CString;

use crate::graphics::{shader::Shader};

use super::gl;

defaults!();

pub struct Program {
    id: u32,
}

impl Program {

    pub fn new(shaders: &Vec<Shader>) -> Result<Program, String> {

        let id = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in shaders {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            error!("failed to link program: {}", error.to_string_lossy());

            return Err(error.to_string_lossy().into_owned());
        }

        unsafe {
            for shader in shaders {
                gl::DetachShader(id, shader.id());
            }
        }

        let program = Program {
            id
        };

        return Ok(program);

    }

    pub fn get_current_program() -> u32 {
        let mut program_id: gl::types::GLint = 0;

        unsafe {
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut program_id as *mut i32);
        }

        return program_id as u32;
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn free(&mut self) {
        info!("Program free");
        if self.id != 0 {
            unsafe {
                gl::DeleteProgram(self.id);
            }
            self.id = 0;
        }
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        debug!("drop program");
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
