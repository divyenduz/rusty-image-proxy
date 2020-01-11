mod process_image;
use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Result};
use std::fs::File;

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
async fn index(query: web::Query<HashMap<String, String>>) -> Result<fs::NamedFile> {
    let link = query.get("link").unwrap();
    let hash = calculate_hash(&link);
    /*
        TODO:
        Move files to static (or entirely remove their need)
        Satan wrote rust because this is valid
          let filename = hash.to_string() + ".png";
        and this isn't
          let filename = "static/" + hash.to_string() + ".png";
    */
    let filename = hash.to_string() + ".png";

    let mut output = File::create(&filename)?;

    // TODO: Download to buffer, don't write to file
    let mut easy = Easy::new();
    easy.url(link).unwrap();
    easy.write_function(move |data| Ok(output.write(data).unwrap()))
        .unwrap();
    easy.perform().unwrap();

    // TODO: Read from an in-memory buffer, not the file system
    let img = image::open(&filename).unwrap();
    let img = process_image::run(img);

    // TODO: Serve the buffer without writing to the file system
    img.save(&filename).unwrap();

    Ok(fs::NamedFile::open(&filename)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
