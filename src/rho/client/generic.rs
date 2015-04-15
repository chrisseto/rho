use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;
use event::InputEvent;
use buffer::Buffer;
use client::Client;

use client::Client;

pub struct GenericClient {
    reciever: Receiver,
    buffers: Arc<RwLock<Vec<Buffer>>>,
}

pub impl Client for GenericClient {
    fn new(buffers: Arc<RwLock<Vec<Buffer>>>, reciever: Receiver) {GenericClient{reciever: reciever, buffers: buffers}}

    fn reciever(&self) {self.reciever}
    fn handle_input(&self, event: InputEvent) {

    }
}
