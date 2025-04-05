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

#[tokio::main]
async fn main() {
    println!("🚀 Container Rust démarré...");

    let config = ini!("C:\\temp\\config.ini");

    let domain_name = config["container_config"]["container_id"].clone().unwrap();
    let multicast_address = config["container_config"]["multicast_address"].clone().unwrap();
    let multicast_port = config["container_config"]["multicast_port"].clone().unwrap();
    let multicast_delay = config["container_config"]["multicast_delay_in_seconds"].clone().unwrap();
    let unicast_port = config["container_config"]["unicast_port"].clone().unwrap();
    let working_folder_path = config["container_config"]["working_folder_path"].clone().unwrap();

    // Exécuter Multicast et Unicast en parallèle
    let multicast_task = tokio::spawn(multicast::send_alive(domain_name, multicast_address, multicast_port.parse().unwrap(), multicast_delay.parse().unwrap(), unicast_port.parse().unwrap()));
    let unicast_task = tokio::spawn(unicast::start_container_server(unicast_port.parse().unwrap(), working_folder_path));

    // Attendre que les deux tâches tournent en parallèle
    let _ = tokio::join!(multicast_task, unicast_task);
}