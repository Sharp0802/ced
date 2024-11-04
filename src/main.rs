mod input;

use crate::input::Input;
use getch_rs::Key;
use nix::poll::{poll, PollFd, PollFlags};
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use std::io::{stdin, Write};
use std::os::fd::AsRawFd;

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical])
        .flex(Flex::Center)
        .areas(area);
    area
}

fn main() {
    /*
    if args().len() != 2 {
        eprintln!("USAGE: ted <name>");
        exit(1);
    }*/

    let getch = getch_rs::Getch::new();
    let stdin = stdin().as_raw_fd();
    let poll_fd = PollFd::new(stdin, PollFlags::POLLIN);


    let mut current_file = "";

    let mut editing_field = Input::multi_line();

    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(72),
                    Constraint::Fill(1),
                ])
                .split(frame.area());

            let p = Paragraph::new(editing_field.get_element())
                .block(Block::bordered()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                    .title(if current_file.len() == 0 { "*" } else { current_file }))
                .wrap(Wrap { trim: true });

            frame.render_widget(p, layout[1]);
        }).unwrap();

        if poll(&mut [ poll_fd ], 0).unwrap() == 0 {
            continue;
        }

        let key = match getch.getch() {
            Ok(key) => key,
            Err(_) => continue,
        };

        editing_field.put_char(&key);

        match key {
            Key::Esc => {
                break;
            }
            Key::Ctrl('s') => {
                /*
                if current_file.len() == 0 {

                    let dialog = Paragraph::new("")
                        .block(Block::bordered()
                            .title("New File")
                            .border_type(BorderType::Double));

                    let file = match File::create_new(current_file) {
                        Ok(file) => file,
                        Err(e) => {}
                    };
                } else {

                }*/
            }
            _ => {}
        }
    }
    ratatui::restore();
}
