use ini::ini;

use std::path::PathBuf;

mod multicast;
mod protocol;
mod unicast;
mod message_analyser;
mod post_manager;
mod protocol_test;
mod get_manager;
mod delete_manager;

#[tokio::main]
async fn main() {
    println!("🚀 Container Rust démarré...");

    // Construction dynamique du chemin
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/config.ini")
        .to_string_lossy()
        .into_owned();

    let config = ini!(&config_path);    

    //let config = ini!("C:\\bloc2\\UE21_Reseau\\projet\\container\\dispatch-container\\src\\config.ini");

    let domain_name = config["container_config"]["container_id"].clone().unwrap();
    let multicast_address = config["container_config"]["multicast_address"].clone().unwrap();
    let multicast_port = config["container_config"]["multicast_port"].clone().unwrap();
    let multicast_delay = config["container_config"]["multicast_delay_in_seconds"].clone().unwrap();
    let dispatcher_ip = config["container_config"]["dispatcher_ip"].clone().unwrap();
    let unicast_port = config["container_config"]["unicast_port"].clone().unwrap();
    let working_folder_path = config["container_config"]["working_folder_path"].clone().unwrap();

    // Exécuter Multicast et Unicast en parallèle
    let multicast_task = tokio::spawn(multicast::send_alive(domain_name, multicast_address, multicast_port, multicast_delay));
    let unicast_task = tokio::spawn(unicast::connect_to_dispatcher(dispatcher_ip, unicast_port, working_folder_path));

    // Attendre que les deux tâches tournent en parallèle
    let _ = tokio::join!(multicast_task, unicast_task);
}
