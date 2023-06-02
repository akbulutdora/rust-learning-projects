use crate::server::Database;
use std::{any::Any, fmt};
use tokio::sync::oneshot;

pub struct Query {
    tx: oneshot::Sender<Box<dyn Any + Send>>,
    f: Box<dyn FnMut(&Database) -> Box<dyn Any + Send> + Send>,
}

impl Query {
    pub fn execute(mut self, database: &Database) {
        let result = (self.f)(database);
        let _ = self.tx.send(result);
    }
    pub fn new<F>(f: F) -> (Self, oneshot::Receiver<Box<dyn Any + Send>>)
    where
        F: FnMut(&Database) -> Box<dyn Any + Send> + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let f = Box::new(f);
        let query = Self { tx, f };

        (query, rx)
    }
}

impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Query").field("tx", &self.tx).finish()
    }
}
