#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate failure;
#[macro_use] extern crate failure_derive;

mod runner;
mod loader;
mod error;

fn main() {
    env_logger::init().expect("Unable to initialize logger");

    runner::run().expect("Unable to run");
   
}