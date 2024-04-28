use napi::{
    bindgen_prelude::{Error, Result},
    Env, Task,
};
use xcap::{image::RgbaImage, Monitor as XCapMonitor, Window as XCapWindow};

use crate::Image;

#[derive(Debug, Clone)]
pub struct XCapWindowWrap(XCapWindow);

unsafe impl Send for XCapWindowWrap {}

impl From<XCapWindow> for XCapWindowWrap {
    fn from(value: XCapWindow) -> Self {
        XCapWindowWrap(value)
    }
}

#[derive(Debug, Clone)]
pub enum AsyncCapture {
    Monitor(XCapMonitor),
    Window(XCapWindowWrap),
}

#[napi]
impl Task for AsyncCapture {
    type Output = RgbaImage;
    type JsValue = Image;

    fn compute(&mut self) -> Result<Self::Output> {
        let capture_image = match self {
            AsyncCapture::Monitor(x_cap_monitor) => x_cap_monitor.capture_image(),
            AsyncCapture::Window(x_cap_window) => x_cap_window.0.capture_image(),
        };

        capture_image.map_err(|err| Error::from_reason(err.to_string()))
    }

    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(Image::from(output))
    }
}
