use napi::{
    bindgen_prelude::{AsyncTask, Error, Result},
    Env, JsBuffer, Task,
};
use std::thread;
use xcap::Window as XCapWindow;

use crate::{
    utils::{bytes_to_buffer, rgba_image_to_bytes},
    Monitor,
};

pub struct AsyncCaptureWindow {
    xcap_window: XCapWindow,
    copy_output_data: Option<bool>,
}

impl AsyncCaptureWindow {
    pub fn new(xcap_window: XCapWindow, copy_output_data: Option<bool>) -> Self {
        AsyncCaptureWindow {
            xcap_window,
            copy_output_data,
        }
    }
}

#[napi]
impl Task for AsyncCaptureWindow {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        let xcap_window = self.xcap_window.to_owned();

        let handle = thread::spawn(move || {
            let rgba_image = xcap_window
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
pub struct Window {
    xcap_window: XCapWindow,
    /// The window id
    #[napi(readonly)]
    pub id: u32,
    /// The window app name
    #[napi(readonly)]
    pub app_name: String,
    /// The window title
    #[napi(readonly)]
    pub title: String,
    /// The window current monitor
    #[napi(readonly)]
    pub current_monitor: Monitor,
    /// The window x coordinate.
    #[napi(readonly)]
    pub x: i32,
    /// The window x coordinate.
    #[napi(readonly)]
    pub y: i32,
    /// The window pixel width.
    #[napi(readonly)]
    pub width: u32,
    /// The window pixel height.
    #[napi(readonly)]
    pub height: u32,
    /// The window is minimized.
    #[napi(readonly)]
    pub is_minimized: bool,
    /// The window is maximized.
    #[napi(readonly)]
    pub is_maximized: bool,
}

#[napi]
impl Window {
    fn new(xcap_window: &XCapWindow) -> Self {
        Window {
            xcap_window: xcap_window.clone(),
            id: xcap_window.id(),
            app_name: xcap_window.app_name().to_string(),
            title: xcap_window.title().to_string(),
            current_monitor: Monitor::new(&xcap_window.current_monitor()),
            x: xcap_window.x(),
            y: xcap_window.y(),
            width: xcap_window.width(),
            height: xcap_window.height(),
            is_minimized: xcap_window.is_minimized(),
            is_maximized: xcap_window.is_maximized(),
        }
    }

    #[napi]
    pub fn all() -> Result<Vec<Window>> {
        let monitors = XCapWindow::all()
            .map_err(|e| Error::from_reason(e.to_string()))?
            .iter()
            .map(Window::new)
            .collect();

        Ok(monitors)
    }

    #[napi]
    pub fn capture_image_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        let rgba_image = self
            .xcap_window
            .capture_image()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        let bytes = rgba_image_to_bytes(rgba_image)?;
        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn capture_image(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncCaptureWindow> {
        AsyncTask::new(AsyncCaptureWindow::new(
            self.xcap_window.clone(),
            copy_output_data,
        ))
    }
}
