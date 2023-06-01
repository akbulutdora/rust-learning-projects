use tokio::sync::mpsc;

/// This implementation works, but I don't know how to make the
/// server run in the background. I think the server blocks the main task.

#[tokio::main]
async fn main() {
    println!("Let's go!");
    let (tx, rx) = mpsc::channel(8);
    // Spawn the client task.
    tokio::spawn(run_client(tx));

    // the server is running, but it blocks
    _run_server(rx).await;
}

// we do not use this function in this version
async fn _run_server(mut rx: mpsc::Receiver<String>) {
    println!("running server");
    while let Some(msg) = rx.recv().await {
        println!("received: {:?}", msg);
    }
}

async fn run_client(tx: mpsc::Sender<String>) {
    // wait for 2 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let msg = String::from("Hello ice cream guy");
    tx.send(msg).await.expect("can not send user on channel");
}
