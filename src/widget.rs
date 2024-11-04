use ratatui::Frame;
use ratatui::prelude::Rect;
use crate::global::Global;

pub trait Widget {
    fn draw(&mut self, frame: &mut Frame, rect: Rect, global: &Global);
}
