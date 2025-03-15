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

    // Exécuter Multicast et Unicast en parallèle
    let multicast_task = tokio::spawn(multicast::send_alive());
    let unicast_task = tokio::spawn(unicast::connect_to_dispatcher());

    // Attendre que les deux tâches tournent en parallèle
    let _ = tokio::join!(multicast_task, unicast_task);
}
