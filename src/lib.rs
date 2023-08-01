#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
    bindgen_prelude::{AsyncTask, Buffer, Error, Result},
    Env, Task,
};

use image::{
    codecs::{bmp::BmpEncoder, jpeg::JpegEncoder, png::PngEncoder},
    Pixel, Rgba, RgbaImage,
};
use screenshots::Screen;
use std::thread;

#[napi]
#[derive(Debug)]
pub struct Image {
    rgba_image: RgbaImage,
}

#[napi]
impl Image {
    fn new(rgba_image: RgbaImage) -> Self {
        Image { rgba_image }
    }

    #[napi]
    pub fn width(&self) -> u32 {
        self.rgba_image.width()
    }

    #[napi]
    pub fn height(&self) -> u32 {
        self.rgba_image.height()
    }

    #[napi]
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let pixel = self.rgba_image.get_pixel(x, y);
        pixel.0
    }

    #[napi]
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: Vec<u8>) {
        let pixel = Rgba::from_slice(&pixel);
        self.rgba_image.put_pixel(x, y, *pixel);
    }

    #[napi]
    pub fn to_bmp(&self) -> Result<Buffer> {
        let mut buffer = Vec::new();
        let encoder = BmpEncoder::new(&mut buffer);

        self.rgba_image
            .write_with_encoder(encoder)
            .map_err(|err| Error::from_reason(err.to_string()))?;
        Ok(buffer.into())
    }

    #[napi]
    pub fn to_png(&self) -> Result<Buffer> {
        let mut buffer = Vec::new();
        let encoder = PngEncoder::new(&mut buffer);

        self.rgba_image
            .write_with_encoder(encoder)
            .map_err(|err| Error::from_reason(err.to_string()))?;
        Ok(buffer.into())
    }

    #[napi]
    pub fn to_jpeg(&self) -> Result<Buffer> {
        let mut buffer = Vec::new();
        let encoder = JpegEncoder::new(&mut buffer);

        self.rgba_image
            .write_with_encoder(encoder)
            .map_err(|err| Error::from_reason(err.to_string()))?;
        Ok(buffer.into())
    }

    #[napi]
    pub fn to_rgba(&self) -> Buffer {
        self.rgba_image.to_vec().into()
    }

    #[napi]
    pub fn save(&self, path: String) -> Result<()> {
        self.rgba_image
            .save(path)
            .map_err(|err| Error::from_reason(err.to_string()))
    }
}

pub struct AsyncCapture {
    screen: Screen,
    area: Option<(i32, i32, u32, u32)>,
}

#[napi]
impl Task for AsyncCapture {
    type Output = Image;
    type JsValue = Image;

    fn compute(&mut self) -> Result<Self::Output> {
        let AsyncCapture { screen, area } = *self;
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

            Ok(Image::new(rgba_image))
        });

        handle
            .join()
            .map_err(|_| Error::from_reason(String::from("Async Capture failed")))?
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
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
    pub fn capture_sync(&self) -> Result<Image> {
        let rgba_image = self
            .screen
            .capture()
            .map_err(|err| Error::from_reason(err.to_string()))?;

        Ok(Image::new(rgba_image))
    }

    #[napi]
    pub fn capture(&self) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture {
            screen: self.screen,
            area: None,
        })
    }

    #[napi]
    pub fn capture_area_sync(&self, x: i32, y: i32, width: u32, height: u32) -> Result<Image> {
        let rgba_image = self
            .screen
            .capture_area(x, y, width, height)
            .map_err(|err| Error::from_reason(err.to_string()))?;

        Ok(Image::new(rgba_image))
    }

    #[napi]
    pub fn capture_area(&self, x: i32, y: i32, width: u32, height: u32) -> AsyncTask<AsyncCapture> {
        AsyncTask::new(AsyncCapture {
            screen: self.screen,
            area: Some((x, y, width, height)),
        })
    }
}
