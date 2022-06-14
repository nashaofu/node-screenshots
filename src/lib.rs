#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AsyncTask, Buffer, Error, Result},
  Task,
};

use screenshots::Screenshots as ScreenshotsRS;
use std::thread;

#[napi]
#[derive(Debug)]
pub struct Screenshots {
  screenshots: ScreenshotsRS,
  pub id: u32,
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub scale: f64,
  pub rotation: f64,
}

pub struct AsyncCapturer {
  screenshots: ScreenshotsRS,
}

#[napi]
impl Task for AsyncCapturer {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let screenshots = self.screenshots;
    let handle = thread::spawn(move || {
      let image = screenshots.capture()?;
      Some(Buffer::from(image.buffer()))
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
  fn new(screenshots: ScreenshotsRS) -> Self {
    let display_info = screenshots.display_info;
    Screenshots {
      screenshots,
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
    ScreenshotsRS::all()
      .iter()
      .map(|&screenshots| Screenshots::new(screenshots))
      .collect()
  }

  #[napi]
  pub fn from_display(id: u32) -> Option<Screenshots> {
    let screenshotss = ScreenshotsRS::all();
    let screenshots = screenshotss
      .iter()
      .find(|capturer| capturer.display_info.id == id)?;

    Some(Screenshots::new(*screenshots))
  }

  #[napi]
  pub fn from_point(x: i32, y: i32) -> Option<Screenshots> {
    let screenshots = ScreenshotsRS::from_point(x, y)?;
    Some(Screenshots::new(screenshots))
  }

  #[napi]
  pub fn capture_sync(&self) -> Option<Buffer> {
    let image = self.screenshots.capture()?;
    Some(Buffer::from(image.buffer()))
  }

  #[napi]
  pub fn capture(&self) -> AsyncTask<AsyncCapturer> {
    AsyncTask::new(AsyncCapturer {
      screenshots: self.screenshots,
    })
  }
}
