#[cfg(test)]
mod tests {
    use crate::protocol::Protocol;

    #[test]
    fn test_parse_post() {
        let long_filename = "a".repeat(50);  // 50 caractères
        let long_hash = "b".repeat(50);      // 50 caractères
        let message = format!("POST {} 12345 {}\nfilecontent", long_filename, long_hash);

        let result = Protocol::parse_post_header(&message);

        assert!(result.is_some(), "Le message POST n'a pas été parsé correctement.");

        if let Some((filename, size, hash)) = result {
            assert_eq!(filename, long_filename, "Le nom du fichier est incorrect.");
            assert_eq!(size, 12345, "La taille du fichier est incorrecte.");
            assert_eq!(hash, long_hash, "Le hash du fichier est incorrect.");
        }
    }

}