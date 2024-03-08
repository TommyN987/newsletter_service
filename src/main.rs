use std::net::TcpListener;

use newsletter_service::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    run(listener)?.await
}
