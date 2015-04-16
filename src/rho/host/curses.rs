use ncurses::*;

use std::char;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Sender;

use host::Host;
use buffer::Buffer;
use event::Modifier;
use event::InputEvent;
use client::generic::GenericClient;


pub struct CursesHost {
    sender: Sender<InputEvent>,
    buffers: Arc<RwLock<Vec<Buffer>>>,
}

pub impl Host for CursesHost {
    fn new(buffers: Arc<RwLock<Vec<Buffer>>>, sender: Sender<InputEvent>) -> Self {CursesHost{sender: sender, buffers: buffers}}

    fn sender(&self) -> Sender<InputEvent> {self.sender}

    fn setup(&self) {
        initscr();
        raw();

        // Allow for extended keyboard (like F1).
        keypad(stdscr, true);
        noecho();
    }

    fn teardown(&self) {
        endwin();
    }

    fn poll(&self) -> InputEvent{
        refresh();
        let ch = getch();
        return InputEvent {
            modifier: Some(Modifier::NONE),
            character: char::from_u32(getch() as u32),
        }
    }
}
