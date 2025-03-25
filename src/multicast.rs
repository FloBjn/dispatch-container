use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;
use socket2::{Socket, Domain, Type, Protocol};
use get_if_addrs::get_if_addrs;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};

/// Fonction pour envoyer les annonces ALIVE en multicast
pub async fn send_alive(
    domain_name: &str,
    multicast_ip: &str,
    port_multicast: u16,
    delay: u64,
    unicast_port: u16,
    network_interface: &str,
) {
    let any_ip = Ipv4Addr::UNSPECIFIED;
    // 1. Trouver l'IP de l'interface demandée
    let local_ip = get_if_addrs()
        .expect("Impossible d'obtenir les interfaces réseau")
        .into_iter()
        .find(|iface| iface.name == network_interface && iface.ip().is_ipv4())
        .map(|iface| match iface.ip() {
            IpAddr::V4(ipv4) => ipv4,
            _ => panic!("Pas une IPv4"),
        })
        .expect("Interface réseau introuvable ou sans IP v4");

    println!(
        "📡 Utilisation de l'interface {} ({}) pour le multicast",
        network_interface, local_ip
    );

    // 2. Créer un socket UDP bas niveau
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .expect("Impossible de créer le socket bas niveau");

    // 3. Lier à l'IP locale (optionnel : 0.0.0.0 ou local_ip)
    socket
        .bind(&SocketAddr::from((any_ip, 0)).into())
        .expect("Impossible de binder le socket sur 0.0.0.0");

    // 4. Définir l'interface de sortie pour le multicast
    socket
        .set_multicast_if_v4(&local_ip)
        .expect("Impossible de définir l'interface multicast");

    // 5. Convertir en socket tokio
    let std_socket: std::net::UdpSocket = socket.into();
    let udp_socket = UdpSocket::from_std(std_socket)
        .expect("Impossible de convertir en UdpSocket tokio");

    // 6. Adresse de destination en SocketAddr
    let target: SocketAddr = format!("{}:{}", multicast_ip, port_multicast)
        .parse()
        .expect("Adresse multicast invalide");

    // 7. Boucle d'envoi
    loop {
        let message = format!("ALIVE {}. {}\r\n", domain_name, unicast_port);
        println!("📤 Envoi du message ALIVE : {}", message);

        let sent = udp_socket
            .send_to(message.as_bytes(), target)
            .await
            .expect("Impossible d'envoyer le message");

        println!("✅ {} octets envoyés à {}", sent, target);

        sleep(Duration::from_secs(delay)).await;
    }
}
