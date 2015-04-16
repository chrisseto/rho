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

pub struct Editor<'a, H: 'a + Host, C: 'a + Client> {
    host: &'a H,
    client: &'a C,
    buffers: Arc<RwLock<Vec<Buffer>>>,
}


pub impl<'a> Editor<'a, CursesHost, GenericClient> {
    pub fn new() -> Editor<'a, CursesHost, GenericClient> {
        let buffers = Arc::new(RwLock::new(vec![Buffer::new()]));
        let (sender, recvr): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();
        let (host, client) = (CursesHost::new(buffers.clone(), sender), GenericClient::new(buffers.clone(), recvr));
        Editor{host: &host, client: &client, buffers: buffers}
    }

    pub fn run(&self) {
        self.client.listen();
        self.host.run();
    }
}
