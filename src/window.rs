use napi::bindgen_prelude::{AsyncTask, Error, Result};
use xcap::Window as XCapWindow;

use crate::{async_capture::AsyncCapture, Image, Monitor};

#[napi]
#[derive(Debug, Clone)]
pub struct Window {
    x_cap_window: XCapWindow,
}

#[napi]
impl Window {
    fn new(x_cap_window: &XCapWindow) -> Self {
        Window {
            x_cap_window: x_cap_window.clone(),
        }
    }

    /// List all windows, sorted by z coordinate.
    #[napi]
    pub fn all() -> Result<Vec<Window>> {
        let monitors = XCapWindow::all()
            .map_err(|e| Error::from_reason(e.to_string()))?
            .iter()
            .map(Window::new)
            .collect();

        Ok(monitors)
    }
}

#[napi]
impl Window {
    /// The window id
    #[napi]
    pub fn id(&self) -> Result<u32> {
        self.x_cap_window
            .id()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window process id
    #[napi]
    pub fn pid(&self) -> Result<u32> {
        self.x_cap_window
            .pid()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window app name
    #[napi]
    pub fn app_name(&self) -> Result<String> {
        self.x_cap_window
            .app_name()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window title
    #[napi]
    pub fn title(&self) -> Result<String> {
        self.x_cap_window
            .title()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window current monitor
    #[napi]
    pub fn current_monitor(&self) -> Result<Monitor> {
        let monitor = self
            .x_cap_window
            .current_monitor()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        Ok(Monitor::new(&monitor))
    }
    /// The window x coordinate.
    #[napi]
    pub fn x(&self) -> Result<i32> {
        self.x_cap_window
            .x()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window y coordinate.
    #[napi]
    pub fn y(&self) -> Result<i32> {
        self.x_cap_window
            .y()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window z coordinate.
    #[napi]
    pub fn z(&self) -> Result<i32> {
        self.x_cap_window
            .z()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window pixel width.
    #[napi]
    pub fn width(&self) -> Result<u32> {
        self.x_cap_window
            .width()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window pixel height.
    #[napi]
    pub fn height(&self) -> Result<u32> {
        self.x_cap_window
            .height()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window is minimized.
    #[napi]
    pub fn is_minimized(&self) -> Result<bool> {
        self.x_cap_window
            .is_minimized()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window is maximized.
    #[napi]
    pub fn is_maximized(&self) -> Result<bool> {
        self.x_cap_window
            .is_maximized()
            .map_err(|err| Error::from_reason(err.to_string()))
    }
    /// The window is focused.
    #[napi]
    pub fn is_focused(&self) -> Result<bool> {
        self.x_cap_window
            .is_focused()
            .map_err(|err| Error::from_reason(err.to_string()))
    }

    /// capture the window image synchronously
    #[napi]
    pub fn capture_image_sync(&self) -> Result<Image> {
        let rgba_image = self
            .x_cap_window
            .capture_image()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        Ok(Image::from(rgba_image))
    }

    /// capture the window image asynchronously
    #[napi]
    pub fn capture_image(&self) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture::Window(self.x_cap_window.clone()))
    }
}
