use crate::query::Query;
use crate::server::{Database, Flavor};
use tokio::sync::mpsc;

async fn query_recipe_exists(tx: &mpsc::Sender<Query>) {
    let (q, response_rx) =
        Query::new(|database: &Database| database.flavor_recipes.contains_key(&Flavor::Chocolate));
    tx.send(q).await.expect("CLIENT; can not send on channel");
    match response_rx.await {
        Ok(value) => println!(
            "CLIENT; I asked if he has chocolate flavor! He said {}",
            value
        ),
        Err(e) => println!("CLIENT; failed to receive response: {:?}", e),
    }
}
async fn query_flavors_in_stock(tx: &mpsc::Sender<Query>) {
    let (q, response_rx) = Query::new(|database: &Database| {
        database
            .flavors_stock
            .iter()
            .map(|x| match x {
                Flavor::Chocolate => "chocolate",
                Flavor::Strawberry => "strawberry",
                Flavor::Vanilla => "vanilla",
                Flavor::Watermelon => "watermelon",
                Flavor::Pistacchio => "pistacchio",
            })
            .collect::<Vec<_>>()
            .join(", ")
    });
    tx.send(q).await.expect("CLIENT; can not send over channel");
    match response_rx.await {
        Ok(value) => println!("CLIENT; I asked what flavors he has! He said {}", value),
        Err(e) => println!("CLIENT; failed to receive response: {:?}", e),
    }
}

pub async fn run_client(tx: mpsc::Sender<Query>) {
    let mut i = 0;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        if tx.is_closed() {
            println!("CLIENT; server is closed");
            return;
        }

        match i {
            0 => query_recipe_exists(&tx).await,
            _ => query_flavors_in_stock(&tx).await,
        }
        i += 1;
    }
}
