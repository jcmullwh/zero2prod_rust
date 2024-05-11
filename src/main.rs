use zero2prod_rust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind to a random port provided by the OS
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener)?.await
}
