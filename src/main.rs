// #![allow(dead_code, unused)]

mod app;
mod defs;
mod error;

use crate::app::App;
use crate::defs::Result;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    simple_logger::init()?;

    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(256);

    event_loop.run_app(&mut app)?;

    Ok(())
}
