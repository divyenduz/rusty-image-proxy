use std::env;
use std::fs;
use std::io;

fn does_folder_exist(folder: &str) -> io::Result<bool> {
    let mut path = env::current_dir()?;
    path.push(folder);
    let metadata = fs::metadata(path)?;
    Ok(metadata.is_dir())
}

pub fn ensure_folder(folder: &str) -> io::Result<()> {
    match does_folder_exist(&folder).is_ok() {
        true => Ok(()),                            // Do nothing
        false => Ok(fs::create_dir_all(&folder)?), // Try to create it
    }
}
