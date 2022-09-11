//
// Application
//

use std::{sync::atomic::{AtomicBool, Ordering}};

use crate::graphics::{
    graphics::{self, Graphics},
    api::{Api}
};

use super::graphics::GraphicsContext;

defaults!();

pub trait Executor {
    fn new(api: &mut dyn Api) -> Result<Self, String> where Self: Sized;
    fn initialize(&mut self, api: &mut dyn Api);
    fn free(&mut self, api: &mut dyn Api);
    fn update_layout(&mut self, api: &mut dyn Api);
    fn update_state(&mut self, api: &mut dyn Api, delta: f32);
    fn update_graphics(&mut self, api: &mut dyn Api);
}

struct Configuration {
    frame_rate: i32
}

struct State {
    running: AtomicBool,
    reference_time: std::time::Instant,
    last_update_time: std::time::Instant,
    last_statistics_time: std::time::Instant,
    time_micros: i64,
    time_seconds: f32,
    frame_counter: u64,
    delta_micros: i64,
    delta_seconds: f32,
    delta_frames: u64,
    avg_frames_per_second: f32,
}

pub struct ApplicationContext {
    _engine_context: GraphicsContext
}

pub struct Application<Exec: Executor> {
    configuration: Configuration,
    state: State,
    engine: graphics::Graphics,
    executor: Exec
}

impl<Exec: Executor> Application<Exec> {

    pub fn new(frame_rate: i32) -> Result<Application<Exec>, String> {

        let configuration = Configuration {
            frame_rate
        };

        let state = State {
            running: AtomicBool::new(false),
            reference_time: std::time::Instant::now(),
            last_update_time: std::time::Instant::now(),
            last_statistics_time: std::time::Instant::now(),
            time_micros: 0,
            time_seconds: 0.0,
            frame_counter: 0,
            delta_micros: 0,
            delta_seconds: 0.0,
            delta_frames: 0,
            avg_frames_per_second: 0.0,
        };

        let mut engine = Graphics::new()?;
        let executor = Exec::new(&mut engine as &mut dyn Api)?;

        let mut app = Application {
            configuration,
            state,
            engine,
            executor
        };

        app.initialize();

        return Ok(app);
    }

    fn initialize(&mut self) {
        self.executor.initialize(&mut self.engine as &mut dyn Api);
    }

    fn free(&mut self) {
        info!("Application free");
        self.executor.free(&mut self.engine as &mut dyn Api);
        self.engine.free();
    }

    fn get_time(&self) -> i64 {
        let tm = std::time::Instant::now().duration_since(self.state.reference_time);
        let tm_micros = std::time::Duration::as_micros(&tm);
        return tm_micros as i64;
    }

    fn sleep(&self, micros: i64) {
        if micros < 1 {
            return;
        }
        let sleep_time = std::time::Duration::from_micros(micros as u64);
        std::thread::sleep(sleep_time);
    }

    fn is_running(&self) -> bool {
        return self.state.running.load(Ordering::Acquire);
    }

    fn set_running(&mut self, running: bool) {
        self.state.running.store(running, Ordering::Release);
    }

    fn update_schedule(&mut self) -> bool {

        let state = &mut self.state;

        let now = std::time::Instant::now();

        {
            // absolute time
            let elapsed = now.duration_since(state.reference_time);
            state.time_micros = elapsed.as_micros() as i64;
            state.time_seconds = elapsed.as_secs_f32();
        }

        {
            // delta time
            let elapsed = now.duration_since(state.last_update_time);
            state.delta_micros = elapsed.as_micros() as i64;
            state.delta_seconds = elapsed.as_secs_f32();
            state.last_update_time = now;
        }

        {
            // statistics
            let elapsed = now.duration_since(state.last_statistics_time);
            let delta = elapsed.as_secs_f32();

            if delta >= 5.0 {
                state.avg_frames_per_second = (state.delta_frames as f32) / delta;
                state.delta_frames = 0;
                state.last_statistics_time = now;
                debug!("fps: {}", state.avg_frames_per_second);
            }
        }

        return true;
    }

    fn update_state(&mut self, delta: f32) -> bool {

        if self.engine.has_layout_changed() {
            self.executor.update_layout(&mut self.engine as &mut dyn Api);
        }

        self.engine.set_frame(self.state.time_seconds, self.state.delta_seconds, self.state.frame_counter);

        self.executor.update_state(&mut self.engine as &mut dyn Api, delta);

        return true;
    }

    fn update_graphics(&mut self) -> bool {

        if self.engine.is_minimized() {
            return true; // ok to not draw
        }

        if !self.engine.begin_draw() {
            return false;
        }

        self.executor.update_graphics(&mut self.engine as &mut dyn Api);

        self.state.delta_frames += 1;
        self.state.frame_counter += 1;

        if !self.engine.end_draw() {
            return false;
        }

        return true;
    }

    pub fn get_context(&self) -> ApplicationContext {
        ApplicationContext {
            _engine_context: self.engine.get_context()
        }
    }

    #[allow(dead_code)]
    fn async_update(&mut self) {

    }

    pub fn run(&mut self) {

        debug!("enter run loop");

        self.set_running(true);

        let configuration = &self.configuration;

        let cycle_time = 1000000i64 / configuration.frame_rate as i64;
        let min_cycle_time = 10000i64;
        let max_sleep_time = 5000i64;
        let mut next_cycle = 0i64;

        'main: loop {

            loop {
                if !self.is_running() {
                    break 'main;
                }

                if !self.engine.process_events() {
                    break 'main;
                }

                let now = self.get_time();
                if now >= next_cycle {
                    next_cycle += cycle_time;
                    if next_cycle < now + min_cycle_time {
                        next_cycle = now + min_cycle_time;
                    }
                    break;
                }

                let sleep_time = next_cycle - now;
                self.sleep(if sleep_time > max_sleep_time { max_sleep_time } else { sleep_time });
            }

            if !self.update_schedule() {
                break 'main;
            }

            if !self.update_state(self.state.delta_seconds) {
                break 'main;
            }

            if !self.update_graphics() {
                break 'main;
            }

        }

        self.set_running(false);

        self.free();

        debug!("exit run loop");
    }

}

impl<Exec: Executor> Drop for Application<Exec> {
    fn drop(&mut self) {
        debug!("drop application");
        self.free();
    }
}
