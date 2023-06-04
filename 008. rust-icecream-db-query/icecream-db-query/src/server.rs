use std::collections::HashMap;
use tokio::{
    sync::mpsc,
    time::{self, Duration, Instant},
};

use crate::query::Query;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Flavor {
    Chocolate,
    Strawberry,
    Vanilla,
    Watermelon,
    Pistacchio,
}

#[derive(Debug)]
pub struct Database<'a> {
    pub flavors_stock: Vec<Flavor>,
    pub flavor_recipes: HashMap<Flavor, Vec<&'a str>>,
}

impl<'a> Database<'a> {
    fn new() -> Self {
        Self {
            flavors_stock: vec![Flavor::Chocolate, Flavor::Strawberry],
            flavor_recipes: HashMap::from([
                (Flavor::Chocolate, vec!["milk", "cocoa"]),
                (Flavor::Pistacchio, vec!["pistacchio", "milk"]),
                (Flavor::Strawberry, vec!["strawberry", "coconut milk"]),
                (Flavor::Vanilla, vec!["vanilla", "almond milk"]),
                (Flavor::Watermelon, vec!["watermelon"]),
            ]),
        }
    }
}

pub async fn run_server(mut rx: mpsc::Receiver<Query>) {
    let mut database = Database::new();

    database.flavors_stock.append(&mut vec![Flavor::Watermelon]);

    println!("SERVER; I will speak of truth! You can ask only two questions!");
    let mut ice_cream_amount = 50;
    let mut interval = time::interval_at(
        Instant::now() + Duration::from_secs(1),
        time::Duration::from_secs(1),
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if ice_cream_amount < 10 {
                    println!("SERVER; enough! This is how much you get from my wisdom.");
                    return;
                }
                ice_cream_amount -= 10;
            }
            Some(query) = rx.recv() => {
                println!("SERVER; received the query.");

                query.execute(&mut database);
                println!("SERVER; executed query.")
            }
            else => {
                println!("SERVER; I don't know what's happening here!");
                break;
            }
        }
    }
}
