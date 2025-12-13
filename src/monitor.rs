use napi::bindgen_prelude::{AsyncTask, Error, Result};
use napi_derive::napi;
use xcap::Monitor as XCapMonitor;

use crate::{async_capture::AsyncCapture, Image};

#[napi]
#[derive(Debug, Clone)]
pub struct Monitor {
  x_cap_monitor: XCapMonitor,
}

#[napi]
impl Monitor {
  pub fn new(x_cap_monitor: &XCapMonitor) -> Self {
    Monitor {
      x_cap_monitor: x_cap_monitor.clone(),
    }
  }

  /// List all monitors
  #[napi]
  pub fn all() -> Result<Vec<Monitor>> {
    let monitors = XCapMonitor::all()
      .map_err(|e| Error::from_reason(e.to_string()))?
      .iter()
      .map(Monitor::new)
      .collect();

    Ok(monitors)
  }

  /// Get the monitor by point
  #[napi]
  pub fn from_point(x: i32, y: i32) -> Option<Monitor> {
    Some(Monitor::new(&XCapMonitor::from_point(x, y).ok()?))
  }
}

#[napi]
impl Monitor {
  /// Unique identifier associated with the screen.
  #[napi]
  pub fn id(&self) -> Result<u32> {
    self
      .x_cap_monitor
      .id()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// Unique identifier associated with the screen.
  #[napi]
  pub fn name(&self) -> Result<String> {
    self
      .x_cap_monitor
      .name()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// The screen x coordinate.
  #[napi]
  pub fn x(&self) -> Result<i32> {
    self
      .x_cap_monitor
      .x()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// The screen x coordinate.
  #[napi]
  pub fn y(&self) -> Result<i32> {
    self
      .x_cap_monitor
      .y()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// The screen pixel width.
  #[napi]
  pub fn width(&self) -> Result<u32> {
    self
      .x_cap_monitor
      .width()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// The screen pixel height.
  #[napi]
  pub fn height(&self) -> Result<u32> {
    self
      .x_cap_monitor
      .height()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// Can be 0, 90, 180, 270, represents screen rotation in clock-wise degrees.
  #[napi]
  pub fn rotation(&self) -> Result<f32> {
    self
      .x_cap_monitor
      .rotation()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// Output device's pixel scale factor.
  #[napi]
  pub fn scale_factor(&self) -> Result<f32> {
    self
      .x_cap_monitor
      .scale_factor()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// The screen refresh rate.
  #[napi]
  pub fn frequency(&self) -> Result<f32> {
    self
      .x_cap_monitor
      .frequency()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
  /// Whether the screen is the main screen
  #[napi]
  pub fn is_primary(&self) -> Result<bool> {
    self
      .x_cap_monitor
      .is_primary()
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  /// Whether the screen is builtin
  #[napi]
  pub fn is_builtin(&self) -> Result<bool> {
    self
      .x_cap_monitor
      .is_builtin()
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  /// Capture image of the monitor synchronously
  #[napi]
  pub fn capture_image_sync(&self) -> Result<Image> {
    let rgba_image = self
      .x_cap_monitor
      .capture_image()
      .map_err(|err| Error::from_reason(err.to_string()))?;

    Ok(Image::from(rgba_image))
  }

  /// Capture image of the monitor asynchronously
  #[napi]
  pub fn capture_image(&self) -> AsyncTask<AsyncCapture> {
    AsyncTask::new(AsyncCapture::Monitor(self.x_cap_monitor.clone()))
  }
}
