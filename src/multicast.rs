use ini::ini;
use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;

/// Fonction pour envoyer les annonces ALIVE en multicast
pub async fn send_alive(domain_name: String, multicast_address: String, multicast_port: String, multicast_delay: String) {
    // Crée un socket UDP qui peut envoyer des messages Multicast
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Impossible de créer le socket UDP");

    socket
        .set_multicast_loop_v4(true)
        .expect("Impossible d'activer le mode multicast");

    let target = format!("{}:{}", multicast_address, multicast_port);

    loop {
        let message = format!("ALIVE {}. {}\r\n", domain_name, multicast_port);
        println!("Envoi du message ALIVE : {}", message);

        socket
            .send_to(message.as_bytes(), &target)
            .await
            .expect("Impossible d'envoyer le message");

        sleep(Duration::from_secs(multicast_delay.parse().unwrap())).await;
    }
}
