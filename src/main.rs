mod process_image;
use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Result};
use std::fs::File;

use std::io::Write;

use curl::easy::Easy;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::env;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<fs::NamedFile> {
    let link = query.get("link");
    match link {
        Some(link) => {
            let hash = calculate_hash(&link);
            let folder = String::from("static/");
            let filename = hash.to_string() + ".png";
            let file_path = folder + &filename;
            let mut output = File::create(&file_path)?;
            // TODO: Download to buffer, don't write to file
            let mut easy = Easy::new();
            easy.url(link).unwrap();
            easy.write_function(move |data| Ok(output.write(data).unwrap()))
                .unwrap();
            easy.perform().unwrap();
            // TODO: Read from an in-memory buffer, not the file system
            let img = image::open(&file_path).unwrap();
            let img = process_image::run(img);
            // TODO: Serve the buffer without writing to the file system
            img.save(&file_path).unwrap();
            Ok(fs::NamedFile::open(&file_path)?)
        }
        None => {
            println!(
                "Error: The get parameter link is not provided, please provide an image as a link to process"
            );
            // TODO: This should be a proper error message, once I figure out union types as return type
            Ok(fs::NamedFile::open("error.png")?)
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8088".to_string());

    host.push_str(":");
    host.push_str(&port);

    println!("Listening on {:?}", host);
    HttpServer::new(|| App::new().service(index))
        .bind(host)?
        .run()
        .await
}
