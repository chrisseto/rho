use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;

use buffer::Buffer;
use client::Client;
use event::InputEvent;


pub struct GenericClient {
    reciever: Receiver<InputEvent>,
    buffers: Arc<RwLock<Vec<Buffer>>>,
}

pub impl Client for GenericClient {
    fn new(buffers: Arc<RwLock<Vec<Buffer>>>, reciever: Receiver<InputEvent>) -> Self {GenericClient{reciever: reciever, buffers: buffers}}

    fn reciever(&self) -> Receiver<InputEvent> {self.reciever}
    fn handle_input(&self, event: InputEvent) {

    }
}
