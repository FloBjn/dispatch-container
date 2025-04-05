use std::path::PathBuf;
use ini::configparser::ini::Ini;
use ini::ini;

mod multicast;
pub mod protocol;
mod unicast;
mod message_analyser;
mod protocol_test;

mod request_manager {
    pub mod delete_manager;
    pub mod get_manager;
    pub mod post_manager;
}

const DOMAIN_NAME: &str = "container01";
const MULTICAST_IP: &str = "224.123.123.123";
const MULTICAST_PORT: u16 = 50204;
const NETWORK_INTERFACE: &str = "{EBDC005E-5146-4169-A51D-BE831C9CE0D8}";
const MULTICAST_DELAY: u64 = 60;
static DIRECTORY: &str = "C:\\TEMP\\CONTAINER";
const UNICAST_PORT: u16 = 1234;


#[tokio::main]
async fn main() {
    println!("Container Rust démarré...");
    // Construction dynamique du chemin
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/config.ini")
        .to_string_lossy()
        .into_owned();
    let config = ini!("C:\\temp\\config.ini");

    let domain_name = config["container_config"]["container_id"].clone().unwrap();
    let multicast_address = config["container_config"]["multicast_address"].clone().unwrap();
    let multicast_port = config["container_config"]["multicast_port"].clone().unwrap();
    let multicast_delay = config["container_config"]["multicast_delay_in_seconds"].clone().unwrap();
    let dispatcher_ip = config["container_config"]["dispatcher_ip"].clone().unwrap();
    let unicast_port = config["container_config"]["unicast_port"].clone().unwrap();
    let working_folder_path = config["container_config"]["working_folder_path"].clone().unwrap();

      // Exécuter Multicast et Unicast en parallèle
    let multicast_task = tokio::spawn(multicast::send_alive(domain_name, multicast_address, multicast_port, multicast_delay, dispatcher_ip));
    let unicast_task = tokio::spawn(unicast::start_container_server(unicast_port, working_folder_path));

    // Attendre que les deux tâches tournent en parallèle
    let _ = tokio::join!(multicast_task, unicast_task);
}
