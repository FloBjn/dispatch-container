use crate::{protocol, request_manager::{get_manager, post_manager, delete_manager}};

/// Analyse un message reçu et effectue l'action correspondante.
pub(crate) fn analyse_message(directory: &str, line: &str) -> String {
    match (
        protocol::Protocol::parse_post(line),
        protocol::Protocol::parse_get(line),
        protocol::Protocol::parse_delete(line),
    ) {
        // Si le message correspond à un POST
        (Some((filename, size, hash, octets)), _, _) => {
            // Sauvegarde le fichier et renvoie la réponse
            let response = post_manager::save_file(directory, filename, size, hash, octets);
            response.to_string()
        }
        // Si le message correspond à un GET
        (_, Some(filename), _) => {
            // Récupère le fichier et renvoie la réponse
            let response = get_manager::get_file(directory, filename);
            response.to_string()
        }
        // Si le message correspond à un DELETE
        (_, _, Some(filename)) => {
            // Supprime le fichier et renvoie la réponse
            let response = delete_manager::delete_file(directory, filename);
            response.to_string()
        }
        // Si aucun des cas n'est valide
        _ => "Message non reconnu\r\n".to_string(),
    }
}