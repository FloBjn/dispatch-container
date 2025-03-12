use crate::{delete_manager, get_manager, post_manager, protocol};

/// Analyse un message reçu et effectue l'action correspondante.
pub(crate) fn analyse_message(directory: &str, line: &String) -> String {
    match (
        protocol::Protocol::parse_post(line),
        protocol::Protocol::parse_get(line),
        protocol::Protocol::parse_delete(line),
    ) {
        (Some((filename, size, hash, octets)), _, _) => {
            // Sauvegarde le fichier et renvoie la réponse
            let response = post_manager::save_file(directory, filename, size, hash, octets);
            response.to_string()
        }
        (Some((filename)), _, _) => {
            // Récupère le fichier et renvoie la réponse
            let response = get_manager::get_file(directory, filename);
            response.to_string()
        }
        (Some((filename)), _, _) => {
            // Supprime le fichier et renvoie la réponse
            let response = delete_manager::delete_file(directory, filename);
            response.to_string()
        }
        _ => "Message non reconnu\n".to_string(),
    }
}