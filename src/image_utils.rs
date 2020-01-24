use image::{ImageFormat, ImageOutputFormat};

pub struct ImageFormatType {
    pub format: ImageFormat,
    pub output_format: ImageOutputFormat,
    pub content_type: String,
}

pub fn get_image_format_type(image_bytes: &[u8]) -> ImageFormatType {
    let format = image::guess_format(image_bytes);
    match format {
        Ok(image::ImageFormat::PNG) => ImageFormatType {
            format: ImageFormat::PNG,
            output_format: ImageOutputFormat::PNG,
            content_type: "image/png".to_owned().to_string(),
        },
        Ok(image::JPEG) => ImageFormatType {
            format: ImageFormat::JPEG,
            output_format: ImageOutputFormat::JPEG(60), // TODO: Understand JPEG quality
            content_type: "image/jpeg".to_owned().to_string(), // TODO: Is there a difference between image/jpeg vs image/jpg
        },
        Ok(image::GIF) => ImageFormatType {
            format: ImageFormat::GIF,
            output_format: ImageOutputFormat::GIF,
            content_type: "image/gif".to_owned().to_string(),
        },
        Ok(_) | Err(_) => ImageFormatType {
            format: ImageFormat::PNG,
            output_format: ImageOutputFormat::PNG,
            content_type: "image/png".to_owned().to_string(),
        }, // Is falling back to PNG alright? Not for image::load_from_memory_with_format for sure, maybe panic?
    }
}
