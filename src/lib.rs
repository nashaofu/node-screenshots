#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AsyncTask, Buffer, Error, Result},
  Task,
};

use screenshots::Screen;
use std::thread;

#[napi]
#[derive(Debug)]
pub struct Screenshots {
  screen: Screen,
  pub id: u32,
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub rotation: f64,
  pub scale_factor: f64,
  pub is_primary: bool,
}

pub struct AsyncCapture {
  screen: Screen,
  area: Option<(i32, i32, u32, u32)>,
}

#[napi]
impl Task for AsyncCapture {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let AsyncCapture { screen, area } = *self;
    let handle = thread::spawn(move || {
      let image = if let Some((x, y, width, height)) = area {
        screen.capture_area(x, y, width, height)?
      } else {
        screen.capture()?
      };
      
      Some(Buffer::from(image.buffer().clone()))
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
  fn new(screen: Screen) -> Self {
    Screenshots {
      screen,
      id: screen.id,
      x: screen.x,
      y: screen.y,
      width: screen.width,
      height: screen.height,
      rotation: screen.rotation as f64,
      scale_factor: screen.scale_factor as f64,
      is_primary: screen.is_primary,
    }
  }
  #[napi]
  pub fn all() -> Vec<Screenshots> {
    Screen::all()
      .iter()
      .map(|&screen| Screenshots::new(screen))
      .collect()
  }

  #[napi]
  pub fn from_display(id: u32) -> Option<Screenshots> {
    let screens = Screen::all();
    let screen = screens.iter().find(|screen| screen.id == id)?;

    Some(Screenshots::new(*screen))
  }

  #[napi]
  pub fn from_point(x: i32, y: i32) -> Option<Screenshots> {
    let screen = Screen::from_point(x, y)?;
    Some(Screenshots::new(screen))
  }

  #[napi]
  pub fn capture_sync(&self) -> Option<Buffer> {
    let image = self.screen.capture()?;
    Some(Buffer::from(image.buffer().clone()))
  }

  #[napi]
  pub fn capture(&self) -> AsyncTask<AsyncCapture> {
    AsyncTask::new(AsyncCapture {
      screen: self.screen,
      area: None,
    })
  }

  #[napi]
  pub fn capture_area_sync(&self, x: i32, y: i32, width: u32, height: u32) -> Option<Buffer> {
    let image = self.screen.capture_area(x, y, width, height)?;
    Some(Buffer::from(image.buffer().clone()))
  }

  #[napi]
  pub fn capture_area(&self, x: i32, y: i32, width: u32, height: u32) -> AsyncTask<AsyncCapture> {
    AsyncTask::new(AsyncCapture {
      screen: self.screen,
      area: Some((x, y, width, height)),
    })
  }
}
