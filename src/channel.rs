use std::{
    sync::mpsc::{Receiver, RecvError, RecvTimeoutError, SendError, Sender, TryRecvError, channel},
    time::Duration,
};

pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> (Self, Self) {
        let (sender_1, receiver_2) = channel();
        let (sender_2, receiver_1) = channel();
        (
            Self {
                sender: sender_1,
                receiver: receiver_1,
            },
            Self {
                sender: sender_2,
                receiver: receiver_2,
            },
        )
    }

    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        self.sender.send(message)
    }

    pub fn receive(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn receive_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    pub fn try_receive(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }
}
