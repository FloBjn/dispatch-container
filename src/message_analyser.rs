use crate::{protocol::Protocol, request_manager::{get_manager, delete_manager}};

/// Analyse un message (hors POST binaire) et effectue l'action correspondante.
pub(crate) fn analyse_message(directory: &str, line: &str) -> String {
    match (
        Protocol::parse_get(line),
        Protocol::parse_delete(line),
    ) {
        // Si le message correspond à un GET
        (Some(filename), _) => {
            let response = get_manager::get_file(directory, &filename);
            response.to_string()
        }
        // Si le message correspond à un DELETE
        (_, Some(filename)) => {
            let response = delete_manager::delete_file(directory, &filename);
            response.to_string()
        }
        // Si aucun des cas n'est valide
        _ => "Message non reconnu\r\n".to_string(),
    }
}
