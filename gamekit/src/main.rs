//
// Main
//

#[macro_export]
macro_rules! defaults {
    () => {
        #[allow(unused_imports)]
        use log::{debug, info, trace, warn, error};
    }
}

use log::LevelFilter;
use simplelog::{Config, TermLogger, TerminalMode, ColorChoice};
use crate::{exec::MyExecutor, graphics::application::{self, ApplicationContext}};

mod exec;
pub mod graphics;
mod entity;

defaults!();

const FRAMERATE: i32 = 120;

fn main() {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    );

    info!("startup");

    let _context: ApplicationContext;

    {
        let app = application::Application::<MyExecutor>::new(FRAMERATE);

        match app {
            Ok(mut app) => {
                _context = app.get_context();
                app.run();
            },
            Err(err) => {
                error!("{}", err);
            }
        };

    }

    info!("shutdown");

}
