mod multicast;
mod protocol;

const DOMAIN_NAME: &str = "container01";
const TCP_PORT: u16 = 1234;

#[tokio::main]
async fn main() {
    println!("Container Rust démarré...");
    // Envoi du message ALIVE en multicast
    multicast::send_alive(DOMAIN_NAME, TCP_PORT).await;
}
