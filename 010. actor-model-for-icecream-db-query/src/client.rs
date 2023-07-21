use tokio::sync::mpsc;

use crate::actor::Actor;

pub struct Client<M> {
    handle: mpsc::UnboundedSender<M>,
}

impl<M> Client<M> {
    pub fn new<A, S>(state: S, action: A) -> Self
    where
        M: Send + 'static,
        S: Send + 'static,
        A: Send + FnMut(&mut S, M) -> () + 'static,
    {
        let (handle, actor) = Actor::new(state, action);
        tokio::spawn(actor.run());
        Self { handle }
    }

    pub fn send(&self, message: M) {
        let _ = self.handle.send(message);
    }
}
