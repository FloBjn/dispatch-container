use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::message_analyser;

/// Lance un serveur TCP pour écouter les connexions du Dispatcher.
pub async fn start_container_server(tcp_port: u16, directory: String) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", tcp_port)).await?;
    let directory = Arc::new(directory); // Partage du répertoire entre les connexions

    println!("📡 Serveur démarré sur le port {}", tcp_port);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Connexion acceptée depuis {}", addr);
                let directory = Arc::clone(&directory);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, &directory).await {
                        eprintln!("⚠️ Erreur avec {} : {}", addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("⚠️ Erreur d'acceptation de connexion : {}", e);
            }
        }
    }
}

/// Gère une connexion avec un client (le Dispatcher).
async fn handle_client(mut stream: TcpStream, directory: &str) -> tokio::io::Result<()> {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("🔴 Déconnexion du client !");
                break;
            }
            Ok(_) => {
                println!("📩 Message reçu : {}", line.trim());
                let response = format!("\n{}", message_analyser::analyse_message(directory, &line));
                writer.write_all(response.as_bytes()).await?;
                line.clear();
            }
            Err(e) => {
                eprintln!("Erreur de lecture du client : {}", e);
                break;
            }
        }
    }
    Ok(())
}