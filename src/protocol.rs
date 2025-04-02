use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POST_HEADER_REGEX: Regex = Regex::new(r"^POST\s+(?P<filename>[A-Fa-f0-9]+)\s+(?P<size>\d+)\s+(?P<hash>[A-Za-z0-9]+)\r?\n$").unwrap();
    pub static ref GET_REGEX: Regex = Regex::new(r"^GET\s+(?P<filename>[A-Fa-f0-9]+)(\r\n)?$").unwrap();
    pub static ref DELETE_REGEX: Regex = Regex::new(r"^DELETE\s+(?P<filename>[A-Za-z0-9._-]+)\n$").unwrap();
}

pub struct Protocol;

impl Protocol {
    pub const EOL: &'static str = "\n";

    /// Parse uniquement l'en-tête POST (sans les octets)
    pub fn parse_post_header(header: &str) -> Option<(String, usize, String)> {
        if let Some(cap) = POST_HEADER_REGEX.captures(header) {
            let filename = cap["filename"].to_string();
            let size = cap["size"].parse::<usize>().ok()?;
            let hash = cap["hash"].to_string();
            Some((filename, size, hash))
        } else {
            None
        }
    }

    pub fn parse_get(message: &str) -> Option<String> {
        GET_REGEX.captures(message).map(|cap| cap["filename"].to_string())
    }

    pub fn parse_delete(message: &str) -> Option<String> {
        DELETE_REGEX.captures(message).map(|cap| cap["filename"].to_string())
    }

    pub fn post_result(success: bool) -> String {
        if success {
            format!("POST_OK{}", Self::EOL)
        } else {
            format!("POST_ERROR{}", Self::EOL)
        }
    }

    pub fn get_result(success: bool, filename: &str, size: usize, encrypted_content: &str) -> String {
        if success {
            format!("GET_OK {} {} {} {}", filename, size, encrypted_content, Self::EOL)
        } else {
            format!("GET_ERROR{}", Self::EOL)
        }
    }

    pub fn delete_result(success: bool) -> String {
        if success {
            format!("DELETE_OK{}", Self::EOL)
        } else {
            format!("DELETE_ERROR{}", Self::EOL)
        }
    }
}