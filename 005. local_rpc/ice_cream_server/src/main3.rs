use tokio::sync::mpsc;

/// This implementation works, but I don't know why exactly
///
/// While trying to learn about this, I read something on the
/// Rust book that excited me:
///     > One increasingly popular approach to ensuring safe
///     concurrency is message passing, where threads or actors
///     communicate by sending each other messages containing data.
///
///      Here’s the idea in a slogan from the Go language documentation:
///         “Do not communicate by sharing memory;
///         instead, share memory by communicating.”
/// Source: https://doc.rust-lang.org/book/ch16-02-message-passing.html
///
/// Also, I understood that `while let Some(msg)` works until the
/// transmitter is dropped.

#[tokio::main]
async fn main() {
    println!("Let's go!");
    let (tx, rx) = mpsc::channel(8);
    // Spawn the client task.
    tokio::spawn(run_client(tx));

    // Spawn the server task in a way it doesnt block the main thread
    tokio::spawn(async move {
        run_server(rx).await;
    })
    .await
    .unwrap();
}

// we do not use this function in this version
async fn run_server(mut rx: mpsc::Receiver<String>) {
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
