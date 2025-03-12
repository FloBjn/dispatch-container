#[cfg(test)]
mod tests {
    use crate::protocol::Protocol;
    use super::*;

    #[test]
    fn test_parse_post_valid() {
        let message = "POST fichier.txt 12 abc123\r\nHello world!";
        let result = Protocol::parse_post(message);

        assert!(result.is_some(), "Le message POST valide doit être parsé");
        let (filename, size, hash, octets) = result.unwrap();

        assert_eq!(filename, "fichier.txt");
        assert_eq!(size, 12);
        assert_eq!(hash, "abc123");
        assert_eq!(octets, "Hello world!");
    }

    #[test]
    fn test_parse_post_invalid_format() {
        let message = "POST fichier.txt abc 12abc123\r\nHello world!";
        let result = Protocol::parse_post(message);

        assert!(result.is_none(), "Le message POST invalide ne doit pas être parsé");
    }

    #[test]
    fn test_parse_post_incorrect_size() {
        let message = "POST fichier.txt 5 abc123\r\nHello world!";
        let result = Protocol::parse_post(message);

        assert!(result.is_some(), "Le message POST doit être parsé");
        let (_, size, _, octets) = result.unwrap();

        assert_ne!(size, octets.len(), "La taille déclarée et le contenu ne correspondent pas");
    }

    #[test]
    fn test_parse_get_valid() {
        let message = "GET fichier.txt\r\n";
        let result = Protocol::parse_get(message);

        assert!(result.is_some(), "Le message GET valide doit être parsé");
        assert_eq!(result.unwrap(), "fichier.txt");
    }

    #[test]
    fn test_parse_get_invalid_format() {
        let message = "GET fichier.txt";
        let result = Protocol::parse_get(message);

        assert!(result.is_none(), "Le message GET invalide ne doit pas être parsé");
    }

    #[test]
    fn test_parse_delete_valid() {
        let message = "DELETE fichier.txt\r\n";
        let result = Protocol::parse_delete(message);

        assert!(result.is_some(), "Le message DELETE valide doit être parsé");
        assert_eq!(result.unwrap(), "fichier.txt");
    }

    #[test]
    fn test_parse_delete_invalid_format() {
        let message = "DELETE fichier.txt";
        let result = Protocol::parse_delete(message);

        assert!(result.is_none(), "Le message DELETE invalide ne doit pas être parsé");
    }
}