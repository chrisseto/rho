#![feature(catch_panic)]
mod curses;

use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Sender;

use buffer::Buffer;
use event::InputEvent;
pub use self::curses::CursesHost;


pub trait Host {
    fn new(Arc<RwLock<Vec<Buffer>>>, Sender<InputEvent>) -> Self;

    fn setup(&self);
    fn teardown(&self);
    fn sender(&self) -> Sender<InputEvent>;
    fn poll(&self) -> InputEvent;

    fn run(&self) -> i32 {
        self.setup();
        let res = thread::catch_panic(|| self._run());
        self.teardown();

        if res.is_ok() {
            return 0;
        }
        return -1;
    }

    fn _run(&self) {
        loop {
            self.sender().send(self.poll())
        };
        return
    }
}
