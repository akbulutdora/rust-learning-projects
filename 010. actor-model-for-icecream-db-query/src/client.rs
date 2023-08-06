use crate::actor::Actor;
use crate::actor::ActorBuilder;
use crate::actor::ActorState;
use crate::tracker::Tracker;
use tokio::sync::mpsc;

use tokio::sync::oneshot;

use tokio::sync::oneshot::channel as oneshot_channel;
pub struct Client<T: ActorState> {
    handle: mpsc::UnboundedSender<T::Message>,
}

impl<T> Client<T>
where
    T: 'static + Send + ActorState,
{
    pub fn new<F, Fut, A>(actor_builder: ActorBuilder<T, A, F, Fut>) -> Self
    where
        A: Send + FnMut(&mut T, T::Message) -> () + 'static,
        F: 'static + Send + Fn(&T) -> Fut,
        Fut: std::future::Future + Send + 'static,
    {
        let handle: mpsc::UnboundedSender<T::Message>;
        let actor: Actor<T, A, F>;

        (handle, actor) = actor_builder.build();
        tokio::spawn(actor.perform());
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
