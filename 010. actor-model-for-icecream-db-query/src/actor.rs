use tokio::sync::mpsc;

pub struct Actor<T, A, M> {
    state: T,
    receiver: mpsc::UnboundedReceiver<M>,
    action: A,
}

impl<T, A, M> Actor<T, A, M>
where
    A: Send + FnMut(&mut T, M) -> (),
{
    pub fn new(state: T, action: A) -> (mpsc::UnboundedSender<M>, Self) {
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
        while let Some(message) = self.receiver.recv().await {
            (self.action)(&mut self.state, message);
        }
    }
}
