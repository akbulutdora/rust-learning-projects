mod client;
mod query;
mod server;
use crate::{client::run_client, server::run_server};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    println!("Let's go!");
    let (tx, rx) = mpsc::channel(8);

    let client = tokio::spawn(run_client(tx));
    let server = tokio::spawn(run_server(rx));

    client.await.unwrap();
    server.await.unwrap();
}
