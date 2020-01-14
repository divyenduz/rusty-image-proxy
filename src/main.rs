mod fs_utils;
mod process_image;
mod runtime_env;

use std::collections::HashMap;

use actix_files as afs;
use actix_web::{get, web, App, HttpServer, Result};
use std::fs;

use std::io::Write;

use curl::easy::Easy;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<afs::NamedFile> {
    let link = query.get("link");
    match link {
        Some(link) => {
            let hash = calculate_hash(&link);

            let folder = String::from("static/");
            fs_utils::ensure_folder(&folder)?;

            // TODO: Support other file types
            let filename = hash.to_string() + ".png";
            let file_path = folder + &filename;
            let mut output = fs::File::create(&file_path)?;
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
            Ok(afs::NamedFile::open(&file_path)?)
        }
        None => {
            println!(
                "Error: The get parameter link is not provided, please provide an image as a link to process"
            );
            // TODO: This should be a proper error message, once I figure out union types as return type
            // TODO: Ensure, error.png is present after the build
            // https://doc.rust-lang.org/std/macro.include_bytes.html (Need to alter the return type)
            // https://github.com/pyros2097/rust-embed
            Ok(afs::NamedFile::open("error.png")?)
        }
    }
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
