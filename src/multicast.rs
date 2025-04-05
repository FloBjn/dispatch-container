use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use crate::protocol::Protocol;

/// Fonction pour envoyer les annonces ALIVE en multicast
pub async fn send_alive(
    domain_name: String,
    multicast_ip: String,
    port_multicast: String,
    delay: String,
    unicast_port: String,
) {
    // 1. Créer un socket UDP simple (lié à 0.0.0.0 pour laisser le système choisir l'interface)
    let udp_socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Impossible de binder le socket");

    // 2. Définir l'adresse cible
    let target: SocketAddr = format!("{}:{}", multicast_ip, port_multicast)
        .parse()
        .expect("Adresse multicast invalide");

    println!("Envoi de multicast vers {} toutes les {} secondes...", target, delay);

    // 3. Boucle d'envoi
    loop {
        let message = Protocol::alive_message(&*domain_name, unicast_port.parse().unwrap());
        println!("Envoi du message ALIVE : {}", message.trim());

        let sent = udp_socket
            .send_to(message.as_bytes(), target)
            .await
            .expect("Impossible d'envoyer le message");

        println!("✅ {} octets envoyés à {}", sent, target);

        sleep(Duration::from_secs(delay.parse().unwrap())).await;
    }
}
