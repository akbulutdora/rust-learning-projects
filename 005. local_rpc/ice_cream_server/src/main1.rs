use tokio::sync::mpsc;

/// This one just doesn't work. Initializing the channel and spawning two
/// tasks seemed intuitive at first, but the program just ends.
/// Resource: https://tokio.rs/tokio/tutorial/channels

#[tokio::main]
pub async fn version_one() {
    println!("Let's go!");
    // Create the channel
    let (tx, rx) = mpsc::channel(8);
    // Spawn the server task.
    tokio::spawn(run_server(rx));

    // Spawn the client task.
    tokio::spawn(run_client(tx));
}

async fn run_server(mut rx: mpsc::Receiver<String>) {
    println!("running server");
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}

async fn run_client(tx: mpsc::Sender<String>) {
    println!("sending from first handle");
    // wait for 2 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("woke up from sleep");

    tx.send("sending from first handle".to_owned())
        .await
        .unwrap();
}
