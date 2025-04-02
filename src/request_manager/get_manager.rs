use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::protocol::Protocol;

pub(crate) fn get_file(directory: &str, file_name: &str) -> String {
    let file_path = Path::new(directory).join(file_name);

    if file_path.exists() {
        // Ouvre le fichier et gère les erreurs correctement
        let mut file = match File::open(&file_path) {
            Ok(f) => f,
            Err(_) => return Protocol::get_result(false, file_name, 0, ""),
        };

        //Lis le contenu et le stock dans un Vec, tout en gérant les erreurs
        let mut content = Vec::new();
        if file.read_to_end(&mut content).is_err() {
            return Protocol::get_result(false, file_name, 0, "");
        }

        let content = match String::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return Protocol::get_result(false, file_name, 0, ""),
        };

        //Va chercher la taille dans les metadata du fichier trouvé
        let size = match file_path.metadata() {
            Ok(meta) => meta.len() as usize,
            Err(_) => return Protocol::get_result(false, file_name, 0, ""),
        };

        return Protocol::get_result(true, file_name, size, &content);
    }

    Protocol::get_result(false, file_name, 0, "")
}