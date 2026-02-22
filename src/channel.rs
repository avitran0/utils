use std::sync::mpsc::{Receiver, Sender};

pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}
