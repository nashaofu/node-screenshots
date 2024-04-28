use napi::bindgen_prelude::{AsyncTask, Error, Result};
use xcap::Window as XCapWindow;

use crate::{async_capture::AsyncCapture, Image, Monitor};

#[napi]
#[derive(Debug, Clone)]
pub struct Window {
    x_cap_window: XCapWindow,
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
    fn new(x_cap_window: &XCapWindow) -> Self {
        Window {
            x_cap_window: x_cap_window.clone(),
            id: x_cap_window.id(),
            app_name: x_cap_window.app_name().to_string(),
            title: x_cap_window.title().to_string(),
            current_monitor: Monitor::new(&x_cap_window.current_monitor()),
            x: x_cap_window.x(),
            y: x_cap_window.y(),
            width: x_cap_window.width(),
            height: x_cap_window.height(),
            is_minimized: x_cap_window.is_minimized(),
            is_maximized: x_cap_window.is_maximized(),
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
    pub fn capture_image_sync(&self) -> Result<Image> {
        let rgba_image = self
            .x_cap_window
            .capture_image()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        Ok(Image::from(rgba_image))
    }

    #[napi]
    pub fn capture_image(&self) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture::Window(self.x_cap_window.clone().into()))
    }
}
