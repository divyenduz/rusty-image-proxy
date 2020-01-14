extern crate image as rs_image;
mod common_utils;
mod fs_utils;
mod process_image;
mod runtime_env;

use std::io::prelude::*;

use std::collections::HashMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let link = query.get("link");
    match link {
        Some(link) => {
            let hash = common_utils::calculate_hash(&link);
            let folder = String::from("static/");
            fs_utils::ensure_folder(&folder)?;

            match fs_utils::download(link) {
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

                            // TODO: Support other formats
                            let filename = hash.to_string() + ".png";
                            let file_path = folder + &filename;

                            image.save(&file_path)?;
                            let mut file = std::fs::File::open(&file_path)?;
                            let mut buffer = Vec::new();
                            file.read_to_end(&mut buffer)?;

                            Ok(HttpResponse::Ok()
                                .content_type(content_type)
                                .body(buffer))
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
