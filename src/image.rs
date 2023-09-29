use napi::{
    bindgen_prelude::{AsyncTask, Buffer, Error, Result},
    Env, Task,
};

use screenshots::image::{
    codecs::{bmp::BmpEncoder, jpeg::JpegEncoder, png::PngEncoder},
    Pixel, Rgba, RgbaImage,
};
use std::{sync::Arc, thread};

#[derive(Clone, Copy)]
enum ImageEncode {
    PNG,
    JPEG,
    BMP,
}

pub struct AsyncEncoder {
    rgba_image: Arc<RgbaImage>,
    image_encode: ImageEncode,
}

#[napi]
impl Task for AsyncEncoder {
    type Output = Buffer;
    type JsValue = Buffer;

    fn compute(&mut self) -> Result<Self::Output> {
        let rgba_image = Arc::clone(&self.rgba_image);
        let image_encode = self.image_encode;

        let handle = thread::spawn(move || {
            let mut buffer = Vec::new();
            let encode_result = match image_encode {
                ImageEncode::PNG => rgba_image.write_with_encoder(PngEncoder::new(&mut buffer)),
                ImageEncode::JPEG => rgba_image.write_with_encoder(JpegEncoder::new(&mut buffer)),
                ImageEncode::BMP => rgba_image.write_with_encoder(BmpEncoder::new(&mut buffer)),
            };

            encode_result.map_err(|err| Error::from_reason(err.to_string()))?;
            Ok(buffer.into())
        });

        handle
            .join()
            .map_err(|_| Error::from_reason(String::from("to failed")))?
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi]
#[derive(Debug, Clone)]
pub struct Image {
    rgba_image: RgbaImage,
}

#[napi]
impl Image {
    pub fn new(rgba_image: RgbaImage) -> Self {
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
    pub fn to_png(&self) -> AsyncTask<AsyncEncoder> {
        AsyncTask::new(AsyncEncoder {
            rgba_image: Arc::new(self.rgba_image.to_owned()),
            image_encode: ImageEncode::PNG,
        })
    }

    #[napi]
    pub fn to_jpeg(&self) -> AsyncTask<AsyncEncoder> {
        AsyncTask::new(AsyncEncoder {
            rgba_image: Arc::new(self.rgba_image.to_owned()),
            image_encode: ImageEncode::JPEG,
        })
    }

    #[napi]
    pub fn to_bmp(&self) -> AsyncTask<AsyncEncoder> {
        AsyncTask::new(AsyncEncoder {
            rgba_image: Arc::new(self.rgba_image.to_owned()),
            image_encode: ImageEncode::BMP,
        })
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
