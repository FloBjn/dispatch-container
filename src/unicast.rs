use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::protocol::Protocol;
use crate::request_manager::post_manager;
use sha2::{Digest, Sha384}; // ← ici on utilise sha2 à la place de sha3
use hex;
use crate::message_analyser::analyse_message;

/// Lance un serveur TCP pour écouter les connexions du Dispatcher.
pub async fn start_container_server(tcp_port: String, directory: String) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", tcp_port)).await?;
    let directory = Arc::new(directory);

    println!("Serveur Container actif sur le port {}", tcp_port);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Connexion acceptée depuis {}", addr);
                let directory = Arc::clone(&directory);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, &directory).await {
                        eprintln!("Erreur avec {} : {}", addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Erreur d'acceptation de connexion : {}", e);
            }
        }
    }
}

async fn handle_client(mut stream: TcpStream, directory: &str) -> tokio::io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut header = String::new();

    reader.read_line(&mut header).await?;
    println!("{:#?}", header);

    if let Some((filename, size, expected_hash)) = Protocol::parse_post_header(&header) {

        // Lecture binaire du fichier
        let mut buffer = vec![0u8; size];
        reader.read_exact(&mut buffer).await?;

        // Vérification du hash (SHA-384 - SHA-2)
        let mut hasher = Sha384::new();
        hasher.update(&buffer);
        let computed_hash = hex::encode(hasher.finalize());

        let success = computed_hash == expected_hash
            && post_manager::save_file(directory, &filename, &buffer);

        let response = Protocol::post_result(success);
        stream.write_all(response.as_bytes()).await?;
    }
    else if Protocol::parse_get(&header).is_some() || Protocol::parse_delete(&header).is_some() {
        // → C'est une commande GET ou DELETE, on appelle analyse_message
        let response = analyse_message(directory, &header);
        stream.write_all(response.as_bytes()).await?;
    }
    else {
        // → Aucun format reconnu
        stream.write_all(b"UNKNOWN_COMMAND\n").await?;
    }

    Ok(())
}
