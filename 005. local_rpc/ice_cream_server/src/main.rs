use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, Duration, Instant};

#[derive(Debug)]
struct ClientRequest {
    message: String,
    tx: oneshot::Sender<String>,
    // TODO: New fields
    // query: ,
}

/*
    Arbitrary operations:
    - how many flavors?
    - what flavors do you have that start with the letter "F"?
    - what kind of chocolate do you have?
    - do you have non-dairy ice cream?
    - are you making more of vanilla?

    https://doc.rust-lang.org/std/ops/trait.FnOnce.html
    keywords: trait objects,
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

async fn run_server(mut rx: mpsc::Receiver<ClientRequest>) {
    println!("SERVER; I will give one person some ice cream!");
    let mut ice_cream_amount = 50;
    let mut interval = time::interval_at(
        Instant::now() + Duration::from_secs(1),
        time::Duration::from_secs(1),
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if ice_cream_amount < 10 {
                    println!("SERVER; no more ice cream!");
                    return;
                }
                ice_cream_amount -= 10;
            }
            Some(ClientRequest { message, tx }) = rx.recv() => {
                println!("SERVER; received: {:?}", message);
                // fn.call(m)
                let response = format!("Here is your ice cream! I have {ice_cream_amount} left!");
                tx.send(response).unwrap();
            }
            else => {
                println!("SERVER; I don't know what's happening here!");
                break;
            }
        }
    }
}

async fn run_client(tx: mpsc::Sender<ClientRequest>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        if tx.is_closed() {
            println!("CLIENT; Ice cream guy is gone! I guess I will go home now.");
            return;
        }
        let (response_tx, response_rx) = oneshot::channel();
        let new_msg = ClientRequest {
            message: String::from("Hello ice cream guy, give me ice cream!"),
            tx: response_tx,
        };
        tx.send(new_msg)
            .await
            .expect("can not send user on channel");

        let answer = response_rx.await.unwrap();
        println!("CLIENT; I knew you loved me!: {:?}", answer);
    }
}
