#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod monitor;
mod utils;
mod window;

pub use monitor::Monitor;
pub use window::Window;
