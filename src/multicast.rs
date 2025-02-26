use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;

/// Adresse et port multicast
const MULTICAST_IP: &str = "224.123.123.123";
const MULTICAST_PORT: u16 = 50204; // Port multicast de notre groupe

/// Fonction pour envoyer les annonces ALIVE en multicast
pub async fn send_alive(domain_name: &str, tcp_port: u16) {
    // Crée un socket UDP qui peut envoyer des messages Multicast
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Impossible de créer le socket UDP");

    socket
        .set_multicast_loop_v4(true)
        .expect("Impossible d'activer le mode multicast");

    let target = format!("{}:{}", MULTICAST_IP, MULTICAST_PORT);

    loop {
        let message = format!("ALIVE {}. {}\r\n", domain_name, tcp_port);
        println!("Envoi du message ALIVE : {}", message);

        socket
            .send_to(message.as_bytes(), &target)
            .await
            .expect("Impossible d'envoyer le message");

        sleep(Duration::from_secs(30)).await;
    }
}
