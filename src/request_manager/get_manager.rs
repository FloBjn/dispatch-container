use std::path::Path;
use crate::protocol;
use crate::protocol::Protocol;

pub(crate) fn get_file(directory: &str, file_name: String) -> String {
    let file_path = Path::new(directory).join(&file_name);
    if file_path.exists() {
        Protocol::get_result(true, &*file_name, file_path.metadata().unwrap().len() as usize, "hash_placeholder")
    } else {
        Protocol::get_result(false, "", 0, "")
    }
}