use tokio::sync::mpsc;

pub trait ActorState {
    type Message: Send;
}

pub struct Actor<T, A>
where
    T: ActorState,
{
    state: T,
    receiver: mpsc::UnboundedReceiver<T::Message>,
    action: A,
}

impl<T, A> Actor<T, A>
where
    T: 'static + Send + ActorState,
    A: Send + FnMut(&mut T, T::Message) -> (),
{
    pub fn new(state: T, action: A) -> (mpsc::UnboundedSender<T::Message>, Self) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (
            sender,
            Self {
                state,
                receiver,
                action,
            },
        )
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            (self.action)(&mut self.state, msg);
        }
    }
}
