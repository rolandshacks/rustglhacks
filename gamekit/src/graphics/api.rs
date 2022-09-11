//
// Public API
//

extern crate sdl2;

use crate::graphics::{graphics::{Graphics, Metrics}, primitives::{DrawMode, Primitives}};

use super::gl;

defaults!();

pub enum BlendMode {
    Normal,
    Additive,
    Multiply
}

pub trait Api {
    fn get_metrics(&self) -> &Metrics;
    fn get_time_seconds(&self) -> f32;
    fn get_delta_seconds(&self) -> f32;
    fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);
    fn clear(&self);
    fn draw_arrays(&self, mode: DrawMode, ofs: usize, count: usize);
    fn draw_elements(&self, mode: DrawMode, count: usize);
    fn draw_elements_instanced(&self, mode: DrawMode, count: usize, num_instances: usize);
    fn get_random(&self) -> f32;
    fn get_random_range(&self, min: f32, max: f32) -> f32;
    fn set_blend_mode(&self, blend_mode: BlendMode);
}

static mut RAND_SEED: i32 = 1;

impl Api for Graphics {

    fn get_metrics(&self) -> &Metrics {
        return &self.metrics;
    }

    fn get_time_seconds(&self) -> f32 {
        return self.metrics.time_seconds;
    }

    fn get_delta_seconds(&self) -> f32 {
        return self.metrics.delta_seconds;
    }

    fn get_random(&self) -> f32 {
        unsafe {
            RAND_SEED = RAND_SEED.wrapping_mul(16807);
            return ((RAND_SEED as f32) * 4.6566129e-010f32).abs();
        }
    }

    fn get_random_range(&self, min: f32, max: f32) -> f32 {

        let dist = max-min;
        return min + self.get_random() * dist;
    }

    fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        Primitives::clear_color(red, green, blue, alpha);
    }

    fn clear(&self) {
        Primitives::clear();
    }

    fn draw_arrays(&self, mode: DrawMode, ofs: usize, count: usize) {
        Primitives::draw_arrays(mode, ofs, count);
    }

    fn draw_elements(&self, mode: DrawMode, count: usize) {
        Primitives::draw_elements(mode, count);
    }

    fn draw_elements_instanced(&self, mode: DrawMode, count: usize, num_instances: usize) {
        Primitives::draw_elements_instanced(mode, count, num_instances);
    }

    fn set_blend_mode(&self, blend_mode: BlendMode) {

        let src;
        let dst;

        match blend_mode {
            BlendMode::Normal => {
                src = gl::SRC_ALPHA;
                dst = gl::ONE_MINUS_SRC_ALPHA;
            },
            BlendMode::Additive => {
                src = gl::SRC_ALPHA;
                dst = gl::ONE;
            },
            BlendMode::Multiply => {
                src = gl::DST_COLOR;
                dst = gl::ZERO;
            }
        }

        unsafe {
            gl::BlendFunc(src, dst);
        }
    }

}
