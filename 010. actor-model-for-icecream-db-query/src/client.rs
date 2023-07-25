use tokio::sync::{
    mpsc,
    oneshot::{self, channel as oneshot_channel},
};

use crate::{
    actor::{Actor, ActorState},
    tracker::Tracker,
};

pub struct Client<T: ActorState> {
    handle: mpsc::UnboundedSender<T::Message>,
}

impl<T> Client<T>
where
    T: 'static + Send + ActorState,
{
    pub fn new<F>(state: T, action: F) -> Self
    where
        F: 'static + Send + FnMut(&mut T, <T as ActorState>::Message),
    {
        let (handle, actor) = Actor::new(state, action);
        tokio::spawn(actor.run());
        Self { handle }
    }

    pub fn send_msg<M, O>(&self, msg: M) -> Tracker<O>
    where
        (oneshot::Sender<O>, M): Into<T::Message>,
    {
        let (send, recv) = oneshot_channel();
        let _ = self.handle.send((send, msg).into());
        Tracker::new(recv)
    }
}
