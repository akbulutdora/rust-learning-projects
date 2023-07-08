use clap::{Parser, Subcommand};
use colored::*;
use reqwest;
use serde_json::Value;

const GET_ENDPOINT: &str = "https://api.publicapis.org/entries";
const POST_ENDPOINT: &str = "https://www.google.com";

#[derive(Parser)]
#[command(author, version)]
/// curld is a downgraded curl, tailored for people like me.
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Performs a GET request
    Get(Get),
    /// Performs a POST request
    Post(Post),
}

#[derive(Parser)]
struct Get {
    #[clap(short, long, default_value = GET_ENDPOINT)]
    url: String,
}

#[derive(Parser)]
struct Post {
    #[clap(short, long, default_value = POST_ENDPOINT)]
    url: String,
    #[clap(short, long)]
    data: String,
}

fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Post(Post { url, data })) => {
            println!("POST to {url} with data: {data}");
            let json: Value = serde_json::from_str(&data).unwrap();
            let body = reqwest::blocking::Client::new()
                .post(url)
                .json(&json)
                .send()?
                .json::<Value>()?;
            let msg = serde_json::to_string_pretty(&body).unwrap().blue();
            println!("{msg}");
        }
        Some(Commands::Get(Get { url })) => {
            println!("GET to {url}");
            let body = reqwest::blocking::get(url)?.json::<Value>()?;
            let msg = serde_json::to_string_pretty(&body).unwrap().blue();
            println!("{msg}");
        }
        _ => println!("Bad user! Enter your command!"),
    }

    Ok(())
}
