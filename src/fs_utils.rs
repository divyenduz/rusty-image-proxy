use std::result::Result;
use std::{env, fs, io};

use curl::easy::Easy;

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

pub fn download(from: &str) -> Result<std::vec::Vec<u8>, curl::Error> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(from).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    Ok(data)
}
