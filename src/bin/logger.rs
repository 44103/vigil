use vigil::monitor;

#[tokio::main]
async fn main() {
    monitor::run().await;
}
