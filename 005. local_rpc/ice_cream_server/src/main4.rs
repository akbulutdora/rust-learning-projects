use tokio::sync::mpsc;

/*
24.05.2023
A discord user in the RustLang channel helped me with this.
Her explanation was as follows:

join! runs two futures concurrently. This is useful if they are lazy
(which usually means they are produced by an async fn or async block).

The futures given by tokio::spawn are uniquely not lazy, as
tokio::spawn immediately starts running a task instead of waiting until
 the task is polled with await or join!

Therefore, one can simply await on the tasks in series instead of
join!ing them.

*/
#[tokio::main]
async fn main() {
    println!("Let's go!");
    let (tx, rx) = mpsc::channel(8);

    let client = tokio::spawn(run_client(tx));
    let server = tokio::spawn(run_server(rx));

    client.await.unwrap();
    server.await.unwrap();
}

// we do not use this function in this version
async fn run_server(mut rx: mpsc::Receiver<String>) {
    println!("running server");
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                println!("received: {:?}", msg);
            }
            else => {
                println!("channel closed");
                break;
            }
        }
    }
}

async fn run_client(tx: mpsc::Sender<String>) {
    // Send the first message
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let msg = String::from("Hello ice cream guy");
    tx.send(msg).await.expect("can not send user on channel");

    // Send the second message
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let msg = String::from("Answer me dammit!");
    tx.send(msg).await.expect("can not send user on channel");
}
