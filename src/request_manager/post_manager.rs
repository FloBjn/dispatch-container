use std::fs::{create_dir_all, File};
use std::io::{Write, Error};
use std::path::Path;

/// Sauvegarde un fichier dans `directory` avec le contenu reçu.
/// Retourne "POST_OK\r\n" en cas de succès, sinon "POST_ERROR\r\n".
pub(crate) fn save_file(directory: &str, filename: String, _size: usize, _hash: String, content: String) -> String {
    let path = Path::new(directory);

    // Création du dossier s'il n'existe pas
    if !path.exists() {
        if let Err(e) = create_dir_all(path) {
            eprintln!("⚠️ Erreur de création du dossier {} : {}", directory, e);
            return "POST_ERROR\r\n".to_string();
        }
    }

    // Chemin complet du fichier
    let file_path = path.join(filename);

    // Écriture du fichier
    match File::create(&file_path).and_then(|mut file| file.write_all(content.as_bytes())) {
        Ok(_) => {
            println!("✅ Fichier {} sauvegardé avec succès !", file_path.display());
            "POST_OK\r\n".to_string()
        }
        Err(e) => {
            eprintln!("⚠️ Erreur lors de la sauvegarde du fichier {} : {}", file_path.display(), e);
            "POST_ERROR\r\n".to_string()
        }
    }
}