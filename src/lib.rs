#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
    bindgen_prelude::{AsyncTask, Error, Result},
    Env, JsBuffer, Task,
};

use screenshots::{
    image::{codecs::png::PngEncoder, RgbaImage},
    Screen,
};
use std::thread;

fn rgba_image_to_bytes(rgba_image: RgbaImage) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let encoder = PngEncoder::new(&mut bytes);

    rgba_image
        .write_with_encoder(encoder)
        .map_err(|err| Error::from_reason(err.to_string()))?;

    Ok(bytes)
}

fn bytes_to_buffer(env: Env, bytes: Vec<u8>, copy_output_data: Option<bool>) -> Result<JsBuffer> {
    let buffer = if copy_output_data == Some(true) {
        env.create_buffer_copy(bytes)?
    } else {
        env.create_buffer_with_data(bytes)?
    };

    Ok(buffer.into_raw())
}

pub struct AsyncCapture {
    screen: Screen,
    area: Option<(i32, i32, u32, u32)>,
    copy_output_data: Option<bool>,
}

#[napi]
impl Task for AsyncCapture {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        let screen = self.screen;
        let area = self.area;

        let handle = thread::spawn(move || {
            let rgba_image = if let Some((x, y, width, height)) = area {
                screen
                    .capture_area(x, y, width, height)
                    .map_err(|err| Error::from_reason(err.to_string()))?
            } else {
                screen
                    .capture()
                    .map_err(|err| Error::from_reason(err.to_string()))?
            };

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
    pub fn all() -> Result<Vec<Screenshots>> {
        let screens = Screen::all()
            .map_err(|e| Error::from_reason(e.to_string()))?
            .iter()
            .map(Screenshots::new)
            .collect();

        Ok(screens)
    }

    #[napi]
    pub fn from_point(x: i32, y: i32) -> Option<Screenshots> {
        let screen = Screen::from_point(x, y).ok()?;

        Some(Screenshots::new(&screen))
    }

    #[napi]
    pub fn capture_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        let rgba_image = self
            .screen
            .capture()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        let bytes = rgba_image_to_bytes(rgba_image)?;
        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn capture(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture {
            screen: self.screen,
            area: None,
            copy_output_data,
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
        copy_output_data: Option<bool>,
    ) -> Result<JsBuffer> {
        let rgba_image = self
            .screen
            .capture_area(x, y, width, height)
            .map_err(|err| Error::from_reason(err.to_string()))?;

        let bytes = rgba_image_to_bytes(rgba_image)?;
        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn capture_area(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        copy_output_data: Option<bool>,
    ) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture {
            screen: self.screen,
            area: Some((x, y, width, height)),
            copy_output_data,
        })
    }
}
