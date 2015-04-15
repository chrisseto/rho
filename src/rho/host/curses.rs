
use ncurses::*;

use std::sync::mpsc::Sender;

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
        return InputEvent{
            modifier: Modifier.NONE,
            character: char::from_u32(getch() as u32).expect("Invalid character"),
        }
    }
}
