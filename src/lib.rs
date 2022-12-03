#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AsyncTask, Error, Result},
  Env, JsBuffer, Task,
};

use screenshots::{Image, Screen};
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
  type Output = Image;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let AsyncCapture { screen, area } = *self;
    let handle = thread::spawn(move || {
      let image = if let Some((x, y, width, height)) = area {
        screen.capture_area(x, y, width, height)?
      } else {
        screen.capture()?
      };

      Some(image)
    });

    let capture_image = match handle.join() {
      Ok(result) => result,
      Err(_) => None,
    };

    match capture_image {
      Some(image) => Ok(image),
      None => Err(Error::from_reason(String::from("Capture failed"))),
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    let buffer = env.create_buffer_copy(output.buffer())?;
    Ok(buffer.into_raw())
  }
}

#[napi]
impl Screenshots {
  fn new(screen: &Screen) -> Self {
    let display_info = screen.display_info;

    Screenshots {
      screen: *screen,
      id: display_info.id,
      x: display_info.x,
      y: display_info.y,
      width: display_info.width,
      height: display_info.height,
      rotation: display_info.rotation as f64,
      scale_factor: display_info.scale_factor as f64,
      is_primary: display_info.is_primary,
    }
  }
  #[napi]
  pub fn all() -> Option<Vec<Screenshots>> {
    let screens = Screen::all()?.iter().map(Screenshots::new).collect();

    Some(screens)
  }

  #[napi]
  pub fn from_display(id: u32) -> Option<Screenshots> {
    let screens = Screen::all()?;
    let screen = screens.iter().find(|screen| screen.display_info.id == id)?;

    Some(Screenshots::new(screen))
  }

  #[napi]
  pub fn from_point(x: i32, y: i32) -> Option<Screenshots> {
    let screen = Screen::from_point(x, y)?;
    Some(Screenshots::new(&screen))
  }

  #[napi]
  pub fn capture_sync(&self, env: Env) -> Option<JsBuffer> {
    let image = self.screen.capture()?;
    let buffer = env.create_buffer_copy(image.buffer()).ok()?;
    Some(buffer.into_raw())
  }

  #[napi]
  pub fn capture(&self) -> AsyncTask<AsyncCapture> {
    AsyncTask::new(AsyncCapture {
      screen: self.screen,
      area: None,
    })
  }

  #[napi]
  pub fn capture_area_sync(
    &self,
    env: Env,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
  ) -> Option<JsBuffer> {
    let image = self.screen.capture_area(x, y, width, height)?;
    let buffer = env.create_buffer_copy(image.buffer()).ok()?;
    Some(buffer.into_raw())
  }

  #[napi]
  pub fn capture_area(&self, x: i32, y: i32, width: u32, height: u32) -> AsyncTask<AsyncCapture> {
    AsyncTask::new(AsyncCapture {
      screen: self.screen,
      area: Some((x, y, width, height)),
    })
  }
}
