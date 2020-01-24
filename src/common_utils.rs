use std::result::Result;

use curl::easy::Easy;

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
