#![deny(clippy::all)]

mod async_capture;
mod image;
mod monitor;
mod window;

pub use image::Image;
pub use monitor::Monitor;
pub use window::Window;
