use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::time::Duration;
use ini::ini;
use tokio::time::sleep;
use crate::{message_analyser, protocol};

/// Se connecte au Dispatcher en TCP et gère les déconnexions proprement.
pub async fn connect_to_dispatcher(dispatcher_ip: String, unicast_port: String, working_folder_path: String) {

    let dispatcher_addr = format!("{}:{}", dispatcher_ip, unicast_port);

    loop {
        println!("📡 Tentative de connexion à {}", dispatcher_addr);

        match TcpStream::connect(&dispatcher_addr).await {
            Ok(mut stream) => {
                println!("✅ Connecté au Dispatcher ({})", dispatcher_addr);

                if let Err(e) = register_container(&mut stream, unicast_port.parse().unwrap()).await {
                    eprintln!("⚠️ Erreur d'envoi : {}", e);
                    continue;
                }

                if let Err(e) = listen_for_messages(stream, &*working_folder_path).await {
                    eprintln!("⚠️ Erreur de communication : {}", e);
                }
            }
            Err(e) => {
                eprintln!("⚠️ Erreur de connexion au Dispatcher : {}", e);
            }
        }

        println!("⏳ Tentative de reconnexion dans 5s...");
        sleep(Duration::from_secs(5)).await;
    }
}

/// Envoie un message d'enregistrement au Dispatcher.
async fn register_container(stream: &mut TcpStream, tcp_port: u16) -> tokio::io::Result<()> {
    let message = format!("REGISTER_CONTAINER {}\n", tcp_port);
    stream.write_all(message.as_bytes()).await?;
    println!("📨 Message envoyé : {}", message.trim());
    Ok(())
}

/// Écoute les messages du Dispatcher et envoie les réponses.
async fn listen_for_messages(mut stream: TcpStream, directory: &str) -> tokio::io::Result<()> {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("🔴 Déconnexion du Dispatcher !");
                break;
            }
            Ok(_) => {
                println!("📩 Réponse reçue : {}", line.trim());
                let response = message_analyser::analyse_message(directory, &line);
                writer.write_all(response.as_bytes()).await?;
                line.clear();
            }
            Err(e) => {
                eprintln!("⚠️ Erreur de lecture depuis le Dispatcher : {}", e);
                break;
            }
        }
    }
    Ok(())
}