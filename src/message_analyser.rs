use crate::protocol;

/// Analyser du message reçu
/// Si le message est un POST, sauvegarder le fichier
/// Si le message est un GET, envoyer le fichier
/// Si le message est un DELETE, supprimer le fichier
pub(crate) fn analyse_message(line: &String) {
    // Analyser le message reçu
    if let Some((filename, size, hash)) = protocol::Protocol::parse_post(&line) {
        println!("POST - Fichier: {}, Taille: {}, Hash: {}", filename, size, hash);
    } else if let Some(filename) = protocol::Protocol::parse_get(&line) {
        println!("GET - Fichier: {}", filename);
    } else if let Some(filename) = protocol::Protocol::parse_delete(&line) {
        println!("DELETE - Fichier: {}", filename);
    } else {
        println!("Message non reconnu");
    }
}