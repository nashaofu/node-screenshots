#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AsyncTask, Buffer, Error, Result},
  Task,
};

use screen_capturer::ScreenCapturer;
use std::thread;

#[napi]
#[derive(Debug)]
pub struct Screenshots {
  screen_capturer: ScreenCapturer,
  pub id: u32,
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub scale: f64,
  pub rotation: f64,
}

pub struct AsyncCapturer {
  screen_capturer: ScreenCapturer,
}

#[napi]
impl Task for AsyncCapturer {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let screen_capturer = self.screen_capturer;
    let handle = thread::spawn(move || {
      let image = screen_capturer.capture()?;
      match image.png() {
        Ok(buffer) => Some(Buffer::from(buffer)),
        Err(_) => None,
      }
    });

    let capture_result = match handle.join() {
      Ok(result) => result,
      Err(_) => None,
    };

    match capture_result {
      Some(buffer) => Ok(buffer),
      None => Err(Error::from_reason(String::from("Capture failed"))),
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
impl Screenshots {
  fn new(screen_capturer: ScreenCapturer) -> Self {
    let display_info = screen_capturer.display_info;
    Screenshots {
      screen_capturer,
      id: display_info.id,
      x: display_info.x,
      y: display_info.y,
      width: display_info.width,
      height: display_info.height,
      scale: display_info.scale as f64,
      rotation: display_info.rotation as f64,
    }
  }
  #[napi]
  pub fn all() -> Vec<Screenshots> {
    ScreenCapturer::all()
      .iter()
      .map(|&screen_capturer| Screenshots::new(screen_capturer))
      .collect()
  }

  #[napi]
  pub fn from_display(id: u32) -> Option<Screenshots> {
    let screen_capturers = ScreenCapturer::all();
    let screen_capturer = screen_capturers
      .iter()
      .find(|capturer| capturer.display_info.id == id)?;

    Some(Screenshots::new(*screen_capturer))
  }

  #[napi]
  pub fn from_point(x: i32, y: i32) -> Option<Screenshots> {
    let screen_capturer = ScreenCapturer::from_point(x, y)?;
    Some(Screenshots::new(screen_capturer))
  }

  #[napi]
  pub fn capture_sync(&self) -> Option<Buffer> {
    let image = self.screen_capturer.capture()?;
    match image.png() {
      Ok(buffer) => Some(Buffer::from(buffer)),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn capture(&self) -> AsyncTask<AsyncCapturer> {
    AsyncTask::new(AsyncCapturer {
      screen_capturer: self.screen_capturer,
    })
  }
}
