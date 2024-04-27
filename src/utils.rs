use napi::{Env, Error, JsBuffer, Result};
use xcap::image::{codecs::png::PngEncoder, RgbaImage};

pub fn rgba_image_to_bytes(rgba_image: RgbaImage) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let encoder = PngEncoder::new(&mut bytes);

    rgba_image
        .write_with_encoder(encoder)
        .map_err(|err| Error::from_reason(err.to_string()))?;

    Ok(bytes)
}

pub fn bytes_to_buffer(
    env: Env,
    bytes: Vec<u8>,
    copy_output_data: Option<bool>,
) -> Result<JsBuffer> {
    let buffer = if copy_output_data == Some(true) {
        env.create_buffer_copy(bytes)?
    } else {
        env.create_buffer_with_data(bytes)?
    };

    Ok(buffer.into_raw())
}
