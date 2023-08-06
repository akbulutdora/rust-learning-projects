pub(crate) use actor_model_for_icecream_db_query::map_example::Cache;
use actor_model_for_icecream_db_query::{
    actor::ActorBuilder, client::Client, map_example::CacheMessage,
};
use tokio::{sync::oneshot::Sender as OneshotSender, time};

use std::{collections::HashMap, time::Duration};

#[tokio::main]
async fn main() {
    pub fn cache_action(_cache: &mut Cache, _msg: CacheMessage) -> () {}

    async fn wait(millis: u64) -> u64 {
        time::sleep(Duration::from_millis(millis)).await;
        millis
    }

    let actor_state = Cache(HashMap::new());
    let actor_builder = ActorBuilder::new(actor_state, cache_action).future(|_| wait(5000));
    let client = Client::new(actor_builder);
    client.send_msg((0, "test".to_string()));
    wait(6000).await;
    client.send_msg((0, |cache: &Cache| cache.0.len()));

    wait(10000).await;
    assert!(true)
}
