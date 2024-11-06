mod editing_widget;
mod input_handler;
mod widget;
mod global;
mod global_widget;

use crate::global::Global;
use crate::global_widget::GlobalWidget;
use crate::input_handler::InputHandler;
use crate::widget::Widget;
use nix::poll::{poll, PollFd, PollFlags};
use std::io::stdin;
use std::os::fd::AsRawFd;


fn main() {
    let getch = getch_rs::Getch::new();
    let stdin = stdin().as_raw_fd();
    let poll_fd = PollFd::new(stdin, PollFlags::POLLIN);

    let mut global = Global::new();
    let mut global_widget = GlobalWidget::new();

    let mut terminal = ratatui::init();
    while !global.shutdown() {
        terminal.draw(|frame| {
            global_widget.draw(frame, frame.area(), &global);

            if poll(&mut [ poll_fd ], 15).unwrap() == 0 {
                return;
            }

            let key = match getch.getch() {
                Ok(key) => key,
                Err(_) => return,
            };

            if global_widget.handle(&key, &global) {
                global.set_shutdown();
            }
        }).unwrap();
    }
    ratatui::restore();
}
