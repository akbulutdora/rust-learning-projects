use futures::future::Ready;
use futures::StreamExt;
use futures::{stream::FuturesUnordered, FutureExt};
pub use std::task::{Context, Poll};
use tokio::sync::mpsc;

pub trait ActorState {
    type Message: Send;
}

pub struct Actor<T, A, F>
where
    T: ActorState,
{
    state: T,
    receiver: mpsc::UnboundedReceiver<T::Message>,
    action: A,
    futures: Vec<F>,
}

impl<T, A, F, Fut> Actor<T, A, F>
where
    T: 'static + Send + ActorState,
    A: Send + FnMut(&mut T, T::Message) -> (),
    F: 'static + Send + Fn(&T) -> Fut,
    Fut: std::future::Future + Send,
{
    pub fn builder(state: T, action: A) -> ActorBuilder<T, A, F, Fut> {
        ActorBuilder::new(state, action)
    }

    pub async fn perform(mut self) -> ! {
        fn mapped_index(x: usize) -> impl FnOnce(()) -> Ready<usize> {
            move |()| futures::future::ready(x)
        }
        // `futures` is a vector of user-defined objects that will generate a `Future`
        // that we can await. This makes the actor "alive" in some sense.

        // call each future that was provided
        let mut futs_unordered: FuturesUnordered<_> = self
            .futures
            .iter()
            .enumerate()
            .map(|(index, fut)| (fut)(&self.state).map(mapped_index(index)))
            .collect();
        loop {
            tokio::select! {
                Some(msg) = self.receiver.recv() => {
                    println!("received!");
                    (self.action)(&mut self.state, msg);
                },
                Some(x) = futs_unordered.next() =>
                    {
                        let index = (x)(()).into_inner();
                        let old_fut = &self.futures[index];
                        let new_fut = (old_fut)(&self.state).map(mapped_index(index));
                        futs_unordered.push(new_fut);
                        println!("got one");
                    },

                // data = fut => {
                //     // <- 2. call the future again so that it will be polled again and again periodically
                //     fut = (&self.get_future)(&self.state);
                //     (self.process_future_data)(&mut self.state, data)
                // }
            }
        }
    }
}

pub struct ActorBuilder<T, A, F, Fut>
where
    T: ActorState,
    F: Fn(&T) -> Fut,
    Fut: std::future::Future + Send,
{
    state: T,
    action: A,
    futures: Vec<F>,
    sender: mpsc::UnboundedSender<T::Message>,
    receiver: mpsc::UnboundedReceiver<T::Message>,
}

impl<T, A, F, Fut> ActorBuilder<T, A, F, Fut>
where
    T: 'static + Send + ActorState,
    A: Send + FnMut(&mut T, T::Message) -> (),
    F: 'static + Send + Fn(&T) -> Fut,
    Fut: std::future::Future + Send,
{
    pub fn new(state: T, action: A) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            state,
            sender,
            receiver,
            action,
            futures: vec![],
        }
    }

    pub fn future<'a>(mut self, get_future: F) -> Self {
        // self.get_future = Some(get_future);
        self.futures.push(get_future);
        self
    }

    pub fn build(self) -> (mpsc::UnboundedSender<T::Message>, Actor<T, A, F>) {
        // let cl = self.clone();
        let actor = Actor {
            state: self.state,
            receiver: self.receiver,
            action: self.action,
            futures: self.futures,
        };
        (self.sender, actor)
    }
}
