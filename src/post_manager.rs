use std::fs::File;
use std::io::Write;

pub(crate) fn save_file(filename: String, size: usize, hash: String) {
    // Créer un fichier avec le nom du fichier
    let mut file = File::create(&filename).expect("Impossible de créer le fichier");

    // Écrire le contenu du fichier
    let content = format!("Fichier de {} octets avec le hash {}", size, hash);

    file.write_all(content.as_bytes())
        .expect("Impossible d'écrire dans le fichier");

    println!("POST - Fichier: {} ({} octets)", filename, size);
}