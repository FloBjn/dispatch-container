use std::io::Write;

pub(crate) fn save_file(filename_hash: String, size_hash: usize, content_hash: String, octets: String) {
    // Créer un fichier avec le nom du fichier
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open(&filename_hash).expect("Impossible de créer ou d'ouvrir le fichier");

    // Écrire le contenu du fichier
    file.write_all(octets.as_bytes()).expect("Impossible d'écrire dans le fichier");

    // Vérifier si le fichier a été correctement écrit
    let file = std::fs::File::open(&filename_hash).expect("Impossible d'ouvrir le fichier");
    let metadata = file.metadata().expect("Impossible de lire les métadonnées du fichier");
    let file_size = metadata.len() as usize;

}