use napi::{
    bindgen_prelude::{AsyncTask, Error, Result},
    Env, JsBuffer, Task,
};
use std::thread;
use xcap::Monitor as XCapMonitor;

use crate::utils::{bytes_to_buffer, rgba_image_to_bytes};

pub struct AsyncCaptureMonitor {
    xcap_monitor: XCapMonitor,
    copy_output_data: Option<bool>,
}

impl AsyncCaptureMonitor {
    pub fn new(xcap_monitor: XCapMonitor, copy_output_data: Option<bool>) -> Self {
        AsyncCaptureMonitor {
            xcap_monitor,
            copy_output_data,
        }
    }
}

#[napi]
impl Task for AsyncCaptureMonitor {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        let xcap_monitor = self.xcap_monitor.to_owned();

        let handle = thread::spawn(move || {
            let rgba_image = xcap_monitor
                .capture_image()
                .map_err(|err| Error::from_reason(err.to_string()))?;

            rgba_image_to_bytes(rgba_image)
        });

        handle
            .join()
            .map_err(|_| Error::from_reason(String::from("Async Capture failed")))?
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        bytes_to_buffer(env, output, self.copy_output_data)
    }
}

#[napi]
#[derive(Debug, Clone)]
pub struct Monitor {
    xcap_monitor: XCapMonitor,
    /// Unique identifier associated with the screen.
    #[napi(readonly)]
    pub id: u32,
    /// Unique identifier associated with the screen.
    #[napi(readonly)]
    pub name: String,
    /// The screen x coordinate.
    #[napi(readonly)]
    pub x: i32,
    /// The screen x coordinate.
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
    pub(crate) fn new(xcap_monitor: &XCapMonitor) -> Self {
        Monitor {
            xcap_monitor: xcap_monitor.clone(),
            id: xcap_monitor.id(),
            name: xcap_monitor.name().to_string(),
            x: xcap_monitor.x(),
            y: xcap_monitor.y(),
            width: xcap_monitor.width(),
            height: xcap_monitor.height(),
            rotation: xcap_monitor.rotation() as f64,
            scale_factor: xcap_monitor.scale_factor() as f64,
            frequency: xcap_monitor.frequency() as f64,
            is_primary: xcap_monitor.is_primary(),
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
    pub fn capture_image_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        let rgba_image = self
            .xcap_monitor
            .capture_image()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        let bytes = rgba_image_to_bytes(rgba_image)?;
        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn capture_image(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncCaptureMonitor> {
        AsyncTask::new(AsyncCaptureMonitor::new(
            self.xcap_monitor.clone(),
            copy_output_data,
        ))
    }
}
