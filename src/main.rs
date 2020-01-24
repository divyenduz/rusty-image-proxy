extern crate image as rs_image;
mod common_utils;
mod image_utils;
mod process_image;
mod runtime_env;

use std::collections::HashMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let debug = runtime_env::get_debug();
    let link = query.get("link");
    match link {
        Some(link) => {
            match common_utils::download(link) {
                Ok(data) => {
                    let image_meta = image_utils::get_image_format_type(&data);
                    if debug {
                        println!("content_type: {:?} format: {:?} output format: {:?}", image_meta.content_type, image_meta.format, image_meta.output_format);
                    }

                    let image = image::load_from_memory_with_format(&data, image_meta.format);

                    match image {
                        Ok(image) => {
                            let image = process_image::run(image);
                            let mut buffer = Vec::new();

                            let write_op = image.write_to(&mut buffer, image_meta.output_format);
                            match write_op {
                                Ok(_) => Ok(HttpResponse::Ok()
                                .content_type(image_meta.content_type)
                                .body(buffer)),
                                Err(_) => return_error("Error: Failed to write image to bytes")
                            }
                        }
                        Err(_) => return_error("Error: Failed to load image from bytes")
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
