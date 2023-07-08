use crate::server::Database;
use std::fmt::Debug;
use tokio::sync::oneshot;

pub struct Query {
    execute_and_send: Box<dyn FnOnce(&Database) + Send>,
}

/// [Query::new] constructor needs to take a closure of type [FnOnce(&Database) -> T + Send + 'static]
/// it should return a receiver as well as the query object
impl Query {
    pub fn from_request<T, F>(f: F) -> (Self, oneshot::Receiver<T>)
    where
        F: FnOnce(&Database) -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let execute_and_send = Box::new(move |database: &Database| {
            let result = f(database);
            let _ = tx.send(result);
        });
        let query = Self { execute_and_send };

        (query, rx)
    }

    pub fn execute(self, database: &Database) {
        (self.execute_and_send)(database);
    }
}

impl Debug for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Query").finish()
    }
}
