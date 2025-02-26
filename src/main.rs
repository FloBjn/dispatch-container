mod multicast;
mod protocol;
mod unicast;
mod message_analyser;

const DOMAIN_NAME: &str = "container01";
const TCP_PORT: u16 = 1234;

#[tokio::main]
async fn main() {
    println!("🚀 Container Rust démarré...");

    // Exécuter Multicast et Unicast en parallèle
    let multicast_task = tokio::spawn(multicast::send_alive(DOMAIN_NAME, TCP_PORT));
    let unicast_task = tokio::spawn(unicast::connect_to_dispatcher("127.0.0.1", TCP_PORT));

    // Attendre que les deux tâches tournent en parallèle
    let _ = tokio::join!(multicast_task, unicast_task);
}
