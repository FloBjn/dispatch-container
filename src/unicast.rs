use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::sleep;
use crate::protocol;

/// Fonction pour se connecter au Dispatcher en TCP
pub async fn connect_to_dispatcher(dispatcher_ip: &str, tcp_port: u16) {
    let dispatcher_addr = format!("{}:{}", dispatcher_ip, tcp_port); // Format : IP:PORT

    loop {
        match TcpStream::connect(&dispatcher_addr).await {
            Ok(mut stream) => {
                println!("🔗 Connecté au Dispatcher ({})", dispatcher_addr);

                let (reader, mut writer) = stream.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                // Envoyer un message d'identification
                let message = format!("REGISTER_CONTAINER {}", tcp_port);
                writer.write_all(message.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();
                println!("📨 Message envoyé : {}", message);

                // Boucle pour envoyer et recevoir des messages
                loop {
                    // Lire et analyser la réponse du Dispatcher
                    while reader.read_line(&mut line).await.unwrap() > 0 {
                        println!("📩 Réponse reçue : {}", line.trim());

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

                        line.clear();
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️ Erreur de connexion au Dispatcher : {}", e);
                sleep(Duration::from_secs(5)).await; // Reconnexion après 5s
            }
        }
    }
}
