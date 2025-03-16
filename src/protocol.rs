use regex::Regex;
use lazy_static::lazy_static;

/// Définition statique des regex (doit être en dehors d'un `impl`)
lazy_static! {
    pub static ref POST_REGEX: Regex = Regex::new(r"^POST\s+(?P<filename>[A-Za-z0-9._-]+)\s+(?P<size>\d+)\s+(?P<hash>[A-Za-z0-9]+)\n(?P<octets>.*)$").unwrap();
    pub static ref GET_REGEX: Regex = Regex::new(r"^GET\s+(?P<filename>[A-Za-z0-9._-]+)\n$").unwrap();
    pub static ref DELETE_REGEX: Regex = Regex::new(r"^DELETE\s+(?P<filename>[A-Za-z0-9._-]+)\n$").unwrap();
}

/// Structure gérant le protocole
pub struct Protocol;

impl Protocol {
    pub const EOL: &'static str = "\n";

    /// Vérifie si un message correspond au format POST
    pub fn parse_post(message: &str) -> Option<(String, usize, String, String)> {
        if let Some(captures) = POST_REGEX.captures(message) {
            let filename = captures["filename"].to_string();
            let size = captures["size"].parse::<usize>().ok()?;
            let hash = captures["hash"].to_string();
            let octets = captures["octets"].to_string();
            Some((filename, size, hash, octets))
        } else {
            None
        }
    }

    /// Vérifie si un message correspond au format GET
    pub fn parse_get(message: &str) -> Option<String> {
        GET_REGEX.captures(message).map(|cap| cap["filename"].to_string())
    }

    /// Vérifie si un message correspond au format DELETE
    pub fn parse_delete(message: &str) -> Option<String> {
        DELETE_REGEX.captures(message).map(|cap| cap["filename"].to_string())
    }

    /// Génère un message `POST_OK` ou `POST_ERROR`
    pub fn post_result(success: bool) -> String {
        if success {
            format!("POST_OK{}", Self::EOL)
        } else {
            format!("POST_ERROR{}", Self::EOL)
        }
    }

    /// Génère un message `GET_OK` ou `GET_ERROR`
    pub fn get_result(success: bool, filename: &str, size: usize, hash: &str) -> String {
        if success {
            format!("GET_OK {} {} {}{}", filename, size, hash, Self::EOL)
        } else {
            format!("GET_ERROR{}", Self::EOL)
        }
    }

    /// Génère un message `DELETE_OK` ou `DELETE_ERROR`
    pub fn delete_result(success: bool) -> String {
        if success {
            format!("DELETE_OK{}", Self::EOL)
        } else {
            format!("DELETE_ERROR{}", Self::EOL)
        }
    }
}
