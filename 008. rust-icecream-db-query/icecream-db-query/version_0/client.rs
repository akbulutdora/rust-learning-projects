use std::any::Any;

use crate::{
    query::Query,
    server::{Database, Flavor},
};
use tokio::sync::mpsc;

pub async fn run_client(tx: mpsc::Sender<Query>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        if tx.is_closed() {
            println!("CLIENT; server is closed");
            return;
        }
        let f: Box<dyn FnMut(&Database) -> Box<dyn Any + Send> + Send> =
            Box::new(move |database: &crate::server::Database| {
                Box::new(database.flavor_recipes.contains_key(&Flavor::Chocolate))
            });
        let (new_msg, response_rx) = Query::new(f);
        tx.send(new_msg)
            .await
            .expect("can not send user on channel");

        match response_rx.await {
            Ok(value) => {
                // Here, `value` is a `Box<dyn Any + Send>`. You'll have to downcast it
                // to the type you know it should be (in this case, `bool`), and handle
                // the case where it's not the type you expected.
                match value.downcast_ref::<bool>() {
                    Some(b) => {
                        println!("CLIENT; received response: {}", b);
                    }
                    None => {
                        println!("CLIENT; received response of unexpected type");
                    }
                }
            }
            Err(e) => {
                println!("CLIENT; failed to receive response: {:?}", e);
            }
        }
    }
}
