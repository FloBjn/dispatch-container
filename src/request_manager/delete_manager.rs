use std::fs;
use std::path::Path;
use crate::protocol::Protocol;

/// Supprime un fichier dans `directory`.
pub(crate) fn delete_file(directory: &str, filename: &str) -> String {
    let path = Path::new(directory).join(filename);
    if path.exists() {
        fs::remove_file(path).unwrap();
        Protocol::delete_result(true)
    } else {
        Protocol::delete_result(false)
    }
}