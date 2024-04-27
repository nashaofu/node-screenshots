use napi::{bindgen_prelude::AsyncTask, Env, Error, JsBuffer, Result, Task};
use xcap::image::{
    codecs::{bmp::BmpEncoder, jpeg::JpegEncoder, png::PngEncoder},
    DynamicImage, RgbaImage,
};

fn bytes_to_buffer(env: Env, bytes: Vec<u8>, copy_output_data: Option<bool>) -> Result<JsBuffer> {
    let buffer = if copy_output_data == Some(true) {
        env.create_buffer_copy(bytes)?
    } else {
        env.create_buffer_with_data(bytes)?
    };

    Ok(buffer.into_raw())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Encoder {
    Png,
    Jpeg,
    Bmp,
    Raw,
}

fn encode_image(rgba_image: &RgbaImage, encoder: Encoder) -> Result<Vec<u8>> {
    if encoder == Encoder::Raw {
        return Ok(rgba_image.to_vec());
    }

    let mut bytes = Vec::new();
    let encoder = match encoder {
        Encoder::Png => rgba_image.write_with_encoder(PngEncoder::new(&mut bytes)),
        Encoder::Jpeg => {
            let dynamic_image = DynamicImage::from(rgba_image.clone());
            let rgb_image = dynamic_image.to_rgb8();
            rgb_image.write_with_encoder(JpegEncoder::new(&mut bytes))
        }
        Encoder::Bmp => rgba_image.write_with_encoder(BmpEncoder::new(&mut bytes)),
        _ => unreachable!(),
    };

    encoder.map_err(|err| Error::from_reason(err.to_string()))?;

    Ok(bytes)
}

pub struct AsyncCropImage {
    rgba_image: RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[napi]
impl Task for AsyncCropImage {
    type Output = DynamicImage;
    type JsValue = Image;

    fn compute(&mut self) -> Result<Self::Output> {
        let dynamic_image = DynamicImage::from(self.rgba_image.clone());

        Ok(dynamic_image.crop_imm(self.x, self.y, self.width, self.height))
    }

    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(Image::from(output.to_rgba8()))
    }
}

pub struct AsyncEncodeImage {
    rgba_image: RgbaImage,
    encoder: Encoder,
    copy_output_data: Option<bool>,
}

#[napi]
impl Task for AsyncEncodeImage {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        encode_image(&self.rgba_image, self.encoder)
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        bytes_to_buffer(env, output, self.copy_output_data)
    }
}

#[napi]
#[derive(Debug, Clone)]
pub struct Image {
    rgba_image: RgbaImage,
    #[napi(readonly)]
    pub width: u32,
    #[napi(readonly)]
    pub height: u32,
}

#[napi]
impl Image {
    #[napi]
    pub fn crop_sync(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let dynamic_image = DynamicImage::from(self.rgba_image.clone());
        Image::from(dynamic_image.crop_imm(x, y, width, height).to_rgba8())
    }

    #[napi]
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> AsyncTask<AsyncCropImage> {
        AsyncTask::new(AsyncCropImage {
            rgba_image: self.rgba_image.clone(),
            x,
            y,
            width,
            height,
        })
    }

    #[napi]
    pub fn to_png_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        let bytes = encode_image(&self.rgba_image, Encoder::Png)?;

        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn to_png(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncEncodeImage> {
        AsyncTask::new(AsyncEncodeImage {
            rgba_image: self.rgba_image.clone(),
            encoder: Encoder::Png,
            copy_output_data,
        })
    }

    #[napi]
    pub fn to_jpeg_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        let bytes = encode_image(&self.rgba_image, Encoder::Jpeg)?;

        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn to_jpeg(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncEncodeImage> {
        AsyncTask::new(AsyncEncodeImage {
            rgba_image: self.rgba_image.clone(),
            encoder: Encoder::Jpeg,
            copy_output_data,
        })
    }

    #[napi]
    pub fn to_bmp_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        let bytes = encode_image(&self.rgba_image, Encoder::Bmp)?;

        bytes_to_buffer(env, bytes, copy_output_data)
    }

    #[napi]
    pub fn to_bmp(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncEncodeImage> {
        AsyncTask::new(AsyncEncodeImage {
            rgba_image: self.rgba_image.clone(),
            encoder: Encoder::Bmp,
            copy_output_data,
        })
    }

    #[napi]
    pub fn to_raw_sync(&self, env: Env, copy_output_data: Option<bool>) -> Result<JsBuffer> {
        bytes_to_buffer(env, self.rgba_image.to_vec(), copy_output_data)
    }

    #[napi]
    pub fn to_raw(&self, copy_output_data: Option<bool>) -> AsyncTask<AsyncEncodeImage> {
        AsyncTask::new(AsyncEncodeImage {
            rgba_image: self.rgba_image.clone(),
            encoder: Encoder::Raw,
            copy_output_data,
        })
    }
}

impl From<RgbaImage> for Image {
    fn from(value: RgbaImage) -> Self {
        Image {
            width: value.width(),
            height: value.height(),
            rgba_image: value,
        }
    }
}
