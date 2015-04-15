mod generic;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub use self::generic::GenericClient;

pub trait Client {
    fn new(Receiver) -> Self;

    fn handle_input(&self);
    fn reciever(&self) -> Receiver;

    fn listen(&self) {
        thread::spawn(|| {
            loop {
                match self.reciever().recv() {
                    Err(e) => break,
                    Ok(event) => self.handle_input(event),
                }
            }
        })
    }
}
