extern crate derive_more;
extern crate image;

mod image_utils;
mod process_image;
mod runtime_env;

use curl::easy::Easy;
use derive_more::Display;
use image_utils::ImageFormatType;
use std::collections::HashMap;

use image::DynamicImage;

use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError, Result};

#[derive(Debug, Display)]
pub enum CustomError {
    #[display(fmt = "Error: Query parameter link is required")]
    ValidationErrorQueryParameterLink,
    #[display(fmt = "Error: Failed to download the image into memory")]
    NetworkErrorCurlDownload,
    #[display(fmt = "Error: Failed to guess image format")]
    ValidationErrorGuessImageFormat,
    #[display(fmt = "Error: Failed to write image")]
    DataErrorFailedToWriteImage,
    #[display(fmt = "Panic: Unknown error occurred")]
    UnknownError,
}

// TODO: Types of str to be idiomatic
fn get_query_param(
    query: web::Query<HashMap<String, String>>,
    str: &str,
) -> Result<String, CustomError> {
    let link = query.get(str);
    match link {
        Some(link) => Ok(link.to_string()),
        None => Err(CustomError::ValidationErrorQueryParameterLink),
    }
}

pub fn download(from: &str) -> Result<std::vec::Vec<u8>, CustomError> {
    let mut data = Vec::new();
    let mut handle = Easy::new();

    // What can make this fail?
    if let Err(_) = handle.url(from) {
        return Err(CustomError::NetworkErrorCurlDownload);
    }

    {
        let mut transfer = handle.transfer();

        if let Err(_) = transfer.write_function(|chunk| {
            data.extend_from_slice(chunk);
            Ok(chunk.len())
        }) {
            return Err(CustomError::NetworkErrorCurlDownload);
        }

        if let Err(_) = transfer.perform() {
            return Err(CustomError::NetworkErrorCurlDownload);
        }
    }
    Ok(data)
}

fn get_image_from_bytes(
    data: Vec<u8>,
    image_meta: &ImageFormatType,
) -> Result<DynamicImage, CustomError> {
    let image = image::load_from_memory_with_format(&data, image_meta.format);
    match image {
        Ok(image) => Ok(image),
        Err(_) => Err(CustomError::ValidationErrorGuessImageFormat),
    }
}

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let debug = runtime_env::get_debug();

    let link = get_query_param(query, &"link")?;

    /* TODO: Look for status code and handle incorrect image URLs properly, like S3 access denied keys and 404s
     * Currently, they fail with CustomError::ValidationGuessImageFormat
     */
    let data = download(&link)?;

    let image_meta = image_utils::get_image_format_type(&data);
    if debug {
        println!(
            "content_type: {:?} format: {:?} output format: {:?}",
            image_meta.content_type, image_meta.format, image_meta.output_format
        );
    }

    let image = get_image_from_bytes(data, &image_meta)?;

    let image = process_image::run(image);
    let mut buffer = Vec::new();

    image
        .write_to(&mut buffer, image_meta.output_format)
        .unwrap(); // TODO: When can this fail?

    Ok(HttpResponse::Ok()
        .content_type(image_meta.content_type)
        .body(buffer))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let bind_to_link = runtime_env::get_bind_to_link();

    println!("Listening on {:?}", bind_to_link);
    HttpServer::new(|| App::new().service(index))
        .bind(bind_to_link)?
        .run()
        .await
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::ValidationErrorQueryParameterLink => {
                println!("Error: Query parameter link is required");
                HttpResponse::BadRequest().body("Error: Query parameter link is required")
            }
            CustomError::NetworkErrorCurlDownload => {
                println!("Error: Failed to download the image into memory");
                HttpResponse::InternalServerError()
                    .body("Error: Failed to download the image into memory")
            }
            CustomError::ValidationErrorGuessImageFormat => {
                println!("Error: Failed to guess image format");
                HttpResponse::InternalServerError().body("Error: Failed to guess image format")
            }
            _ => {
                println!("Panic: Unknown error occurred");
                HttpResponse::InternalServerError().body("Panic: Unknown error occurred")
            }
        }
    }
}
