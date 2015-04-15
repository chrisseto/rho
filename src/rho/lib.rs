#![crate_name = "rho"]
extern crate ncurses;

mod host;
mod event;
mod client;
mod buffer;

use std::sync::Arc;
use std::sync::mpsc;
use std::sync::RwLock;
use std::sync::mpsc::{Receiver, Sender};

use host::Host;
use client::Client;
use buffer::Buffer;
use event::InputEvent;

use host::CursesHost;
pub use client::GenericClient;

pub struct Editor {
    host: Host,
    client: Client,
    buffers: Arc<RwLock<Vec<Buffer>>>,
}


pub impl Editor {
    pub fn new() {
        let buffers = RwLock::new(vec![Buffer::new()]);
        let (sender, recvr): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();
        let (host, client) = (CursesHost::new(buffers, sender), GenericClient::new(buffers,recvr));
    }

    pub fn run(&self) {
        self.client.listen();
        self.host.run();
    }
}
