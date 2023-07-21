use message::Message;

mod actor;
mod client;
mod message;
pub mod tracker;

#[tokio::main]
async fn main() {
    // impl 1. for actor and client
    // The client spawns the Actor task as well.
    let client = client::Client::new(0, display);

    // impl 2. for actor and client
    // The ActorBuilder creates the Actor and client, and spawns the Actor when called.
    // let client = actor::ActorBuilder::create().spawn();

    let data = 5;
    let (tracker, message) = Message::create(&data);
    client.send(message);

    let result = tracker.await;
    match result {
        Ok(val) => println!("tracker: {val}"),
        Err(err) => println!("tracker: err: {}", err),
    }
}

fn display(state: &mut i32, message: Message<i32>) {
    println!("message: {}", message.data);
    let result = state.clone();
    message.sender.send(result).unwrap();
}
