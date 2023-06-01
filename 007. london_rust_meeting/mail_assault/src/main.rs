use std::net::TcpListener;

use mail_assault::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    run(listener)?.await
}
