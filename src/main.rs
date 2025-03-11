mod multicast;
mod protocol;
mod unicast;
mod message_analyser;
mod post_manager;

const DOMAIN_NAME: &str = "container01";
const MULTICAST_IP: &str = "224.123.123.123";
const MULTICAST_PORT: u16 = 50204; // Port multicast de notre groupe
const NETWORK_INTERFACE: &str = "eth0";
const MULTICAST_DELAY: u64 = 60;
const FILE_NAME: &str = "C:\\TEMP\\CONTAINER1";
const UNICAST_PORT: u16 = 1235;


#[tokio::main]
async fn main() {
    println!("🚀 Container Rust démarré...");

    // Exécuter Multicast et Unicast en parallèle
    let multicast_task = tokio::spawn(multicast::send_alive(DOMAIN_NAME, MULTICAST_IP, MULTICAST_PORT, MULTICAST_DELAY));
    let unicast_task = tokio::spawn(unicast::connect_to_dispatcher("127.0.0.1", UNICAST_PORT, FILE_NAME));

    // Attendre que les deux tâches tournent en parallèle
    let _ = tokio::join!(multicast_task, unicast_task);
}
