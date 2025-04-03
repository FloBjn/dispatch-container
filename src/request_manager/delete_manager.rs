use std::fs;
use std::path::Path;
use crate::protocol::Protocol;

pub(crate) fn delete_file(directory: &str, filename: &str) -> String {
    let path = Path::new(directory).join(filename);
    match fs::remove_file(&path) {
        Ok(_) => Protocol::delete_result(true),
        Err(_) => Protocol::delete_result(false),
    }
}
