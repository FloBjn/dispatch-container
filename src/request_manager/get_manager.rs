use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::protocol;
use crate::protocol::Protocol;

pub(crate) fn get_file(directory: &str, file_name: String) -> String {
    let file_path = Path::new(directory).join(&file_name);
    if file_path.exists() /*&& */{
        let mut file = match File::open(&file_path){
            Ok(file) => file,
            Err(_) => return Protocol::get_result(false, "", 0, ""),
        };
        let mut buffer = Vec::new();
        if let Err(_) = file.read_to_end(&mut buffer) {
            return Protocol::get_result(false, "", 0, "");
        }

        let mut content = match String::from_utf8(buffer){
            Ok(content) => content,
            Err(_) => return Protocol::get_result(false, "", 0, ""),
        };

        let mut size = match file_path.metadata(){
            Ok(metadata) => metadata.len() as usize,
            Err(_) => return Protocol::get_result(false, "", 0, ""),
        };

        Protocol::get_result(true, &*file_name, size, &content)
    } else {
        Protocol::get_result(false, "", 0, "")
    }
}