use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Enregistre le fichier et retourne `true` si tout s’est bien passé
pub fn save_file(directory: &str, filename: &str, content: &[u8]) -> bool {
    let path = Path::new(directory).join(filename);


    if let Err(e) = fs::create_dir_all(directory) {
        eprintln!("Erreur création dossier : {}", e);
        return false;
    }

    match File::create(&path).and_then(|mut file| file.write_all(content)) {
        Ok(_) => {
            println!("Fichier sauvegardé avec succès.");
            true
        }
        Err(e) => {
            eprintln!("Erreur lors de la sauvegarde : {}", e);
            false
        }
    }
}