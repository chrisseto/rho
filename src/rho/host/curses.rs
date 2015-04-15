use ncurses::*;

use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::RwLock;
use event::Modifier;
use event::InputEvent;
use client::generic::GenericClient;
use buffer::Buffer;
use host::Host;

use host::Host;
use buffer::Buffer;

pub struct CursesHost {
    sender: Sender,
    buffers: Arc<RwLock<Vec<Buffer>>>,
}

pub impl Host for CursesHost {
    fn new(buffers: Arc<RwLock<Vec<Buffer>>>, sender: Sender) {GenericClient{sender: sender, buffers: buffers}}

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

    fn poll(&self) {
        refresh();
        let ch = getch();
        return InputEvent {
            modifier: Modifier::NONE,
            character: char::from_u32(getch() as u32).expect("Invalid character"),
        }
    }
}
