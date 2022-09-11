//
// Engine
//

extern crate sdl2;

defaults!();

use std::{ptr::null, os::raw::c_void, ffi::{CStr}};

use crate::graphics::primitives::{Primitives};

use super::gl;

#[derive(Default)]

pub struct Metrics {
    pub width: u32,
    pub height: u32,
    pub time_seconds: f32,
    pub delta_seconds: f32,
    pub frame_counter: u64
}

impl Metrics {
    pub fn new() -> Self {
        return Metrics {
            width: 0,
            height: 0,
            time_seconds: 0.0,
            delta_seconds: 0.0,
            frame_counter: 0
        }
    }
}

pub struct GraphicsContext {
    _gl: ()
}

pub enum VSyncMode {
    Synchronous,
    Asynchronous,
    Adaptive
}

pub struct Graphics {
    _sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    gl: (),
    event_pump: sdl2::EventPump,
    viewport_changed: bool,
    layout_changed: bool,
    pub metrics: Metrics
}

extern "system"
fn gl_debug_callback_handler(
    _source: gl::types::GLenum,
    _category: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut c_void
) {
    let msg =  unsafe { CStr::from_ptr(message) };
    let s = msg.to_str().unwrap_or_default();

    let t = "opengl";
    let f = format!("[{}-{}] {}", t, id, s);

    match severity {
        gl::DEBUG_SEVERITY_HIGH => { error!(target: t, "{}", f)  },
        gl::DEBUG_SEVERITY_MEDIUM => { warn!(target: t, "{}", f)  },
        gl::DEBUG_SEVERITY_LOW => { info!(target: t, "{}", f)  },
        _ =>  { debug!(target: t, "{}", f)  }
    }

    //print!("[opengl] {}/{}/{}/{}", source, category, id, severity);
}

impl Graphics {

    pub fn new() -> Result<Graphics, String> {

        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        let window = video_subsystem
            .window("Window", 800, 600)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context()?;

        let gl = gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(gl_debug_callback_handler), null());
            gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, null(), gl::TRUE);
        }

        let event_pump = sdl.event_pump()?;

        let mut engine = Graphics {
            _sdl: sdl,
            video_subsystem,
            window,
            _gl_context,
            gl,
            event_pump,
            viewport_changed: false,
            layout_changed: false,
            metrics: Metrics::new()
        };

        engine.initialize();

        return Ok(engine);
    }

    pub fn free(&mut self) {
        info!("Engine free");
    }

    pub fn get_context(&self) -> GraphicsContext {
        GraphicsContext {
            _gl: self.gl.clone()
        }
    }

    pub fn is_minimized(&self) -> bool {
        if (self.window.window_flags() & (sdl2::sys::SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32)) != 0 {
            return true;
        }

        return false;
    }

    fn initialize(&mut self) {

        self.viewport_changed = true;

        self.update_viewport();
        self.set_vsync_mode(VSyncMode::Adaptive);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);
        }

        Primitives::clear_color(0.0, 0.0, 0.0, 0.0);

    }

    pub fn set_frame(&mut self, time_seconds: f32, delta_seconds: f32, frame_counter: u64) {
        self.metrics.time_seconds = time_seconds;
        self.metrics.delta_seconds = delta_seconds;
        self.metrics.frame_counter = frame_counter;
    }

    fn update_viewport(&mut self) {

        if !self.viewport_changed {
            return;
        }

        let ( w, h ) = self.window.drawable_size();

        self.metrics.width = w;
        self.metrics.height = h;

        Primitives::viewport(0, 0, w, h);

        self.layout_changed = true;

        info!("Updated viewport");

        self.viewport_changed = false;
    }

    pub fn process_events(&mut self) -> bool {

        let event_pump = &mut self.event_pump;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => { return false },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => { return false },
                sdl2::event::Event::Window {timestamp: _, window_id: _, win_event} => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(..) => { self.viewport_changed = true; },
                        _ => {}
                    }
                },
                _ => {},
            }
        }

        if self.viewport_changed {
            self.update_viewport();
        }

        return true;
    }

    pub fn has_layout_changed(&mut self) -> bool {
        let changed = self.layout_changed;
        self.layout_changed = false;
        return changed;
    }

    pub fn begin_draw(&mut self) -> bool {
        return true;
    }

    pub fn end_draw(&mut self) -> bool {

        self.window.gl_swap_window();

        return true;
    }

    pub fn set_vsync_mode(&self, vsync_mode: VSyncMode) {
        let _result = match vsync_mode {
            VSyncMode::Synchronous => { self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync) },
            VSyncMode::Asynchronous => { self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate) },
            VSyncMode::Adaptive => {
                let mut r = self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::LateSwapTearing);
                if r.is_err() {
                    // adaptive mode is not supported, switch to synchronous mode
                    r = self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
                }
                r
            }
        };
    }

    pub fn get_vsync_mode(&self) -> VSyncMode {
        let mode = self.video_subsystem.gl_get_swap_interval();
        match mode {
            sdl2::video::SwapInterval::Immediate => VSyncMode::Asynchronous,
            sdl2::video::SwapInterval::VSync => VSyncMode::Synchronous,
            sdl2::video::SwapInterval::LateSwapTearing => VSyncMode::Adaptive
        }
    }

}

impl Drop for Graphics {
    fn drop(&mut self) {
        debug!("drop engine");
        self.free();
    }
}
