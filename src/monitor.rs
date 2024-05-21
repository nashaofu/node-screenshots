use napi::bindgen_prelude::{AsyncTask, Error, Result};
use xcap::Monitor as XCapMonitor;

use crate::{async_capture::AsyncCapture, Image};

#[napi]
#[derive(Debug, Clone)]
pub struct Monitor {
    x_cap_monitor: XCapMonitor,
    /// Unique identifier associated with the screen.
    #[napi(readonly)]
    pub id: u32,
    /// Unique identifier associated with the screen.
    #[napi(readonly)]
    pub name: String,
    /// The screen x coordinate.
    #[napi(readonly)]
    pub x: i32,
    /// The screen y coordinate.
    #[napi(readonly)]
    pub y: i32,
    /// The screen pixel width.
    #[napi(readonly)]
    pub width: u32,
    /// The screen pixel height.
    #[napi(readonly)]
    pub height: u32,
    /// Can be 0, 90, 180, 270, represents screen rotation in clock-wise degrees.
    #[napi(readonly)]
    pub rotation: f64,
    /// Output device's pixel scale factor.
    #[napi(readonly)]
    pub scale_factor: f64,
    /// The screen refresh rate.
    #[napi(readonly)]
    pub frequency: f64,
    /// Whether the screen is the main screen
    #[napi(readonly)]
    pub is_primary: bool,
}

#[napi]
impl Monitor {
    pub(crate) fn new(x_cap_monitor: &XCapMonitor) -> Self {
        Monitor {
            x_cap_monitor: x_cap_monitor.clone(),
            id: x_cap_monitor.id(),
            name: x_cap_monitor.name().to_string(),
            x: x_cap_monitor.x(),
            y: x_cap_monitor.y(),
            width: x_cap_monitor.width(),
            height: x_cap_monitor.height(),
            rotation: x_cap_monitor.rotation() as f64,
            scale_factor: x_cap_monitor.scale_factor() as f64,
            frequency: x_cap_monitor.frequency() as f64,
            is_primary: x_cap_monitor.is_primary(),
        }
    }

    #[napi]
    pub fn all() -> Result<Vec<Monitor>> {
        let monitors = XCapMonitor::all()
            .map_err(|e| Error::from_reason(e.to_string()))?
            .iter()
            .map(Monitor::new)
            .collect();

        Ok(monitors)
    }

    #[napi]
    pub fn from_point(x: i32, y: i32) -> Option<Monitor> {
        Some(Monitor::new(&XCapMonitor::from_point(x, y).ok()?))
    }

    #[napi]
    pub fn capture_image_sync(&self) -> Result<Image> {
        let rgba_image = self
            .x_cap_monitor
            .capture_image()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        Ok(Image::from(rgba_image))
    }

    #[napi]
    pub fn capture_image(&self) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture::Monitor(self.x_cap_monitor.clone()))
    }
}
