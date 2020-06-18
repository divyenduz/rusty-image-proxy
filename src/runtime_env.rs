use std::env;

pub fn get_debug() -> bool {
    let debug = env::var("DEBUG").unwrap_or("false".to_string());
    debug.parse().unwrap_or(false)
}

pub fn get_bind_to_link() -> String {
    let mut host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8088".to_string());

    host.push_str(":");
    host.push_str(&port);
    host
}
