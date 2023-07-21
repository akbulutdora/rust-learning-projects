use tokio::sync::oneshot;

use crate::tracker::Tracker;

pub struct Message<T> {
    pub data: T,
    pub sender: oneshot::Sender<T>,
}

impl Message<i32> {
    pub fn create(data: &i32) -> (Tracker<i32>, Self) {
        let (sender, receiver) = oneshot::channel();
        let message = Self {
            data: *data,
            sender,
        };
        let tracker = Tracker::new(receiver);
        (tracker, message)
    }
}
