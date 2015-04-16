mod generic;

use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;

use buffer::Buffer;
use event::InputEvent;
pub use self::generic::GenericClient;


pub trait Client {
    fn new(Arc<RwLock<Vec<Buffer>>>, Receiver<InputEvent>) -> Self;

    fn handle_input(&self, InputEvent);
    fn reciever(&self) -> Receiver<InputEvent>;

    fn listen(&self) {
        thread::spawn(|| {
            loop {
                match self.reciever().recv() {
                    Err(e) => break,
                    Ok(event) => self.handle_input(event),
                }
            }
        });
        return
    }
}
