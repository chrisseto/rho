#![feature(catch_panic)]
mod curses;

use std::thread;
use std::sync::mpsc::Sender;

use event::InputEvent;

pub use self::curses::CursesHost;


pub trait Host {
    fn new() -> Self;

    fn setup(&self);
    fn teardown(&self);
    fn sender(&self) -> Sender;
    fn poll(&self) -> InputEvent;

    fn run(&self) -> i32 {
        self.setup();
        let res = thread::catch_panic(self._run);
        self.teardown();

        if res.is_ok() {
            return 0;
        }
        return -1;
    }

    fn _run(&self) {
        loop {
            self.sender().Send(self.poll())
        }
    }
}
