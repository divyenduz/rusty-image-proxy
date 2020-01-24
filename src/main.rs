extern crate image as rs_image;
mod common_utils;
mod process_image;
mod runtime_env;

use image::ImageOutputFormat;
use std::collections::HashMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let link = query.get("link");
    match link {
        Some(link) => {
            match common_utils::download(link) {
                Ok(data) => {
                    let format = image::guess_format(&data);
                    let content_type = match format {
                        Ok(image::ImageFormat::PNG) => "image/png",
                        Ok(_) => "image/png", // TODO: Handle other types
                        Err(msg) => panic!("Error: Failed to determine format from bytes, {:?}", msg)
                    };
                    println!("content_type: {:?} format: {:?}", content_type, format);

                    // TODO: use load_from_memory_with_format variant in future
                    let image = image::load_from_memory(&data);

                    match image {
                        Ok(image) => {
                            let image = process_image::run(image);
                            let mut buffer = Vec::new();

                            let output_format = match format {
                                Ok(image::ImageFormat::PNG) => ImageOutputFormat::PNG,
                                Ok(_) => ImageOutputFormat::PNG, // TODO: Handle other types
                                Err(msg) => panic!("Error: Failed to determine format from bytes, {:?}", msg)
                            };

                            let write_op = image.write_to(&mut buffer, output_format);
                            match write_op {
                                Ok(_) => Ok(HttpResponse::Ok()
                                .content_type(content_type)
                                .body(buffer)),
                                Err(_) => panic!("Error: Failed to write image to bytes")
                            }
                        }
                        Err(msg) => panic!("Error: Failed to load image from bytes, {:?}", msg)
                    }
                }
                _ => return_error("Error: Failed to download the file to the file system")
            }
        }
        None => {
            return_error("Error: The get parameter link is not provided, please provide an image as a link to process")
        }
    }
}

fn return_error(msg: &str) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{:?}", msg)))
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
