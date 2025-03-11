use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::sleep;
use crate::{message_analyser, protocol};

/// Fonction pour se connecter au Dispatcher en TCP et gérer les déconnexions proprement.
pub async fn connect_to_dispatcher(dispatcher_ip: &str, tcp_port: u16, file_name: &str) {
    let dispatcher_addr = format!("{}:{}", dispatcher_ip, tcp_port);

    loop {
        println!("📡 Tentative de connexion à {}", dispatcher_addr);

        match TcpStream::connect(&dispatcher_addr).await {
            Ok(mut stream) => {
                println!("✅ Connecté au Dispatcher ({})", dispatcher_addr);

                let (reader, mut writer) = stream.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                // Envoyer un message d'enregistrement
                let message = format!("REGISTER_CONTAINER {}", tcp_port);
                if let Err(e) = writer.write_all(message.as_bytes()).await {
                    eprintln!("⚠️ Erreur d'envoi : {}", e);
                    continue;
                }
                if let Err(e) = writer.write_all(b"\n").await {
                    eprintln!("⚠️ Erreur d'envoi : {}", e);
                    continue;
                }
                println!("📨 Message envoyé : {}", message);

                // Boucle pour écouter en continu les messages du Dispatcher
                loop {
                    match reader.read_line(&mut line).await {
                        Ok(0) => {
                            println!("🔴 Déconnexion du Dispatcher !");
                            break; // Quitte la boucle et tente de se reconnecter
                        }
                        Ok(_) => {
                            println!("📩 Réponse reçue : {}", line.trim());

                            // Analyse du message reçu
                            message_analyser::analyse_message(file_name, &line);

                            line.clear();
                        }
                        Err(e) => {
                            eprintln!("⚠️ Erreur de lecture depuis le Dispatcher : {}", e);
                            break; // Déconnecte et tente de se reconnecter
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️ Erreur de connexion au Dispatcher : {}", e);
            }
        }

        // Attendre avant de tenter une reconnexion
        println!("⏳ Tentative de reconnexion dans 5s...");
        sleep(Duration::from_secs(5)).await;
    }
}