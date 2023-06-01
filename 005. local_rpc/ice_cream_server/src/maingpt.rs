use std::error::Error;
use std::io::{self, BufRead};
use tokio::sync::mpsc;
use tokio::time::Duration;

/// This program creates a server task that manages a state
/// (the amount of ice cream) and processes commands from a client.
/// The client task reads lines from standard input and sends
/// commands to the server based on the input. The server responds
/// to each command by sending back the current amount of ice cream.
/// The server also melts 10 grams of ice cream every 10 seconds.

// Define an enumeration for commands that the server should understand.
// In this case, we only have one command: GetIceCream
enum ServerCommand {
    GetIceCream(mpsc::UnboundedSender<u32>), // This command also carries a sender for the response.
}

// Define a server struct to hold any state that the server needs.
// In this case, it's the amount of ice cream left.
struct Server {
    ice_cream: u32,
}

impl Server {
    // This function starts a new server and returns a sender for commands to the server.
    async fn start() -> mpsc::UnboundedSender<ServerCommand> {
        let (tx, rx) = mpsc::unbounded_channel::<ServerCommand>();
        let server = Server { ice_cream: 1000 }; // Start the server with 1000 grams of ice cream.

        // Run the server in a separate task so it doesn't block the main task.
        tokio::spawn(server.run_server(rx));

        tx // Return the sender for commands to the server.
    }

    // This function runs the server, taking commands from the given receiver.
    async fn run_server(mut self, mut rx: mpsc::UnboundedReceiver<ServerCommand>) {
        // Setup a tick interval to melt the ice cream.
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            tokio::select! {
                // If the interval ticks, melt some ice cream.
                _ = interval.tick() => {
                    if self.ice_cream > 10 {
                        self.ice_cream -= 10;
                    } else {
                        self.ice_cream = 0;
                    }
                }
                // If a command is received, process it.
                Some(ServerCommand::GetIceCream(response_tx)) = rx.recv() => {
                    // Send the current amount of ice cream to the requester.
                    let _ = response_tx.send(self.ice_cream);
                }
            }
        }
    }
}

// This is the client task. It sends commands to the server based on user input.
async fn client(server_tx: mpsc::UnboundedSender<ServerCommand>) {
    let stdin = io::stdin();
    // Read lines from standard input.
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        // If the line is "get ice cream", send a GetIceCream command to the server.
        if line == "get ice cream" {
            // Create a channel for the response.
            let (response_tx, mut response_rx) = mpsc::unbounded_channel::<u32>();

            // Send the command.
            let _ = server_tx.send(ServerCommand::GetIceCream(response_tx));

            // Wait for and print the response.
            match response_rx.recv().await {
                Some(ice_cream) => {
                    println!("Received {} grams of ice cream.", ice_cream);
                }
                None => {
                    println!("Server didn't respond with any ice cream.");
                }
            }
        }
    }
}

// The main function starts the server and client tasks.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start the server.
    let server_tx = Server::start().await;

    // Run the client task.
    client(server_tx).await;

    Ok(())
}
