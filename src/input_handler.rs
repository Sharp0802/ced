use crate::global::Global;
use getch_rs::Key;

pub trait InputHandler {
    fn handle(&mut self, c: &Key, _global: &Global) -> bool;
}
