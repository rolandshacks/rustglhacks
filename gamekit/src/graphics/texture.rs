//
// Texture
//

use std::os::raw;
use std::path::Path;
use std::fs::File;

use crate::graphics::gl;

use super::primitives::Primitives;

extern crate png;

defaults!();

#[allow(dead_code)]
pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    bits_per_pixel: i32,
    bytes_per_line: i32,
    size: usize,
}

impl Texture {

    pub fn new(file_path: &str) -> Result<Texture, String> {

        const PIXEL_ALIGNMENT: usize = 1;

        let path = Path::new(file_path);
        let file = File::open(path);
        let file = match file {
            Ok(file) => file,
            Err(err) => { return Err(err.to_string()); }
        };

        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut input_buffer = vec![0u8; reader.output_buffer_size()];

        let decoder_status = reader.next_frame(&mut input_buffer);
        let _decoder_status = match decoder_status {
            Ok(output_info) => output_info,
            Err(_) => { return Err("failed to decode bitmap file".to_string()); }
        };
        let info = reader.info();

        let (input_bits_per_pixel, output_bits_per_pixel, gl_internal_type, gl_format) = match info.color_type {
            png::ColorType::Rgb => (24,24, gl::RGB, gl::RGB),
            png::ColorType::Rgba => (32,32, gl::RGBA, gl::RGBA),
            png::ColorType::Grayscale => (8,24, gl::RGB, gl::RGB),
            png::ColorType::GrayscaleAlpha => (16,32, gl::RGBA, gl::RGBA),
            png::ColorType::Indexed => (8,24, gl::RGB, gl::RGB),
            //_ => unreachable!("uncovered color type"),
        };

        let width = info.width as usize;
        let height = info.height as usize;
        let input_bytes_per_line = width * input_bits_per_pixel / 8;
        let input_size = height * input_bytes_per_line;

        debug!("texture {}: {}x{}x{}x{}x{}",
               file_path, width, height, input_bits_per_pixel, input_bytes_per_line, input_size);

        let output_bytes_per_line = get_aligned_size((width * output_bits_per_pixel / 8) as usize, PIXEL_ALIGNMENT);
        let output_size = (height as usize * output_bytes_per_line) as usize;

        let mut output_buffer;
        let mut use_copy = false;

        if input_bytes_per_line != output_bytes_per_line || input_bits_per_pixel != output_bits_per_pixel {

            use_copy = true;
            output_buffer = vec![0u8; output_size];

            for y in 0..height {

                let mut src = y * input_bytes_per_line;
                let mut dest = y * output_bytes_per_line;

                match info.color_type {
                    png::ColorType::Rgb | png::ColorType::Rgba => {
                        unsafe {
                            let src_ptr = input_buffer.as_mut_ptr().offset(src as isize);
                            let dest_ptr = output_buffer.as_mut_ptr().offset(dest as isize);
                            std::ptr::copy_nonoverlapping(src_ptr, dest_ptr, input_bytes_per_line);
                        }
                    },
                    png::ColorType::Grayscale => {

                        for _x in 0..width {

                            let luminance = input_buffer[src];

                            output_buffer[dest+0] = luminance;
                            output_buffer[dest+1] = luminance;
                            output_buffer[dest+2] = luminance;

                            src += 1;
                            dest += 3;
                        }

                    },
                    png::ColorType::GrayscaleAlpha => {

                        for _x in 0..width {

                            let luminance = input_buffer[src+0];
                            let alpha = input_buffer[src+1];

                            output_buffer[dest+0] = luminance;
                            output_buffer[dest+1] = luminance;
                            output_buffer[dest+2] = luminance;
                            output_buffer[dest+3] = alpha;

                            src += 2;
                            dest += 4;
                        }

                    },
                    png::ColorType::Indexed => {

                        let palette = info.palette.as_ref().unwrap();
                        let palette_size = palette.len();

                        let mut r;
                        let mut g;
                        let mut b;

                        for _x in 0..width {
                            let palette_index = input_buffer[src] as usize;
                            let ofs = palette_index * 3;

                            if ofs + 2 < palette_size {
                                r = palette[ofs];
                                g = palette[ofs+1];
                                b = palette[ofs+2];
                            } else {
                                r = 0;
                                g = 0;
                                b = 0;
                            }

                            output_buffer[dest+0] = r;
                            output_buffer[dest+1] = g;
                            output_buffer[dest+2] = b;

                            src += 1;
                            dest += 3;
                        }

                    },
                    // _ => {},
                };
            }
        } else {
            output_buffer = vec![0u8; 0];
        }

        let mut id: gl::types::GLuint = 0;
        unsafe {

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, PIXEL_ALIGNMENT as i32);
            gl::PixelStorei(gl::PACK_ALIGNMENT, PIXEL_ALIGNMENT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as gl::types::GLint);

            let pixels = (if use_copy { output_buffer.as_ptr() } else { input_buffer.as_ptr() }) as *const raw::c_void;

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl_internal_type as gl::types::GLint,
                width as i32,
                height as i32,
                0,
                gl_format,
                gl::UNSIGNED_BYTE,
                pixels
            );

            let result = gl::GetError();
            if result != gl::NO_ERROR {
                return Err("failed: TexImage2D".to_string());
            }

            gl::GenerateMipmap(gl::TEXTURE_2D);
            //gl::BindTexture(gl::TEXTURE_2D, 0);
        };

        let texture = Texture {
            id: id as u32,
            width: width as i32,
            height: height as i32,
            bits_per_pixel: output_bits_per_pixel as i32,
            bytes_per_line: output_bytes_per_line as i32,
            size: output_size as usize
        };

        return Ok(texture);

    }

    pub fn bind(&self, bind_location: u32) {
        Primitives::bind_texture(self.id, bind_location);
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        Primitives::unbind_texture(self.id);
    }

    pub fn free(&mut self) {
        info!("Texture free");
        if self.id != 0 {
            unsafe {
                gl::DeleteTextures(1, &mut self.id);
            }
            self.id = 0;
        }
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

}

impl Drop for Texture {
    fn drop(&mut self) {
        debug!("drop texture");
        self.free();
    }
}

fn get_aligned_size(sz: usize, alignment: usize) -> usize {
    if alignment < 2 { return sz; }
    let remainder = sz % alignment;
    let aligned_size = if remainder != 0 { sz + alignment - remainder } else { sz };
    return aligned_size;
}
