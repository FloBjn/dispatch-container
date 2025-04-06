use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::protocol::Protocol;
use crate::request_manager::post_manager;
use sha2::{Digest, Sha384};
use hex;
use sha3::Sha3_384;
use crate::message_analyser::analyse_message;

pub async fn start_container_server(tcp_port: u16, directory: String) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", tcp_port)).await?;
    let directory = Arc::new(directory);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
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
    // Étape 1 : Lire la ligne de commande
    let mut header = String::new();
    {
        let mut reader = BufReader::new(&mut stream);
        reader.read_line(&mut header).await?;
    }

    if let Some((filename, size, expected_hash)) = Protocol::parse_post_header(&header) {
        // Étape 2 : signaler qu’on est prêt à recevoir les octets
        stream.write_all(b"READY\r\n").await?;

        // Étape 3 : lire les données binaires
        let mut buffer = vec![0u8; size];
        let mut reader = BufReader::new(&mut stream); // ✅ re-emprunt possible
        reader.read_exact(&mut buffer).await?;

        // Étape 4 : vérification et sauvegarde
        let mut hasher = Sha384::new();
        hasher.update(&buffer);
        let computed_hash = hex::encode(hasher.finalize());

        let success = computed_hash == expected_hash
            && post_manager::save_file(directory, &filename, &buffer);

        let response = Protocol::post_result(success);
        stream.write_all(response.as_bytes()).await?;
    }
    else if Protocol::parse_get(&header).is_some() || Protocol::parse_delete(&header).is_some() {
        let response = analyse_message(directory, &header);
        stream.write_all(response.as_bytes()).await?;
    }
    else {
        stream.write_all(b"UNKNOWN_COMMAND\r\n").await?;
    }

    Ok(())
}

