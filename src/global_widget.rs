use std::fs::File;
use std::io::Write;
use crate::editing_widget::EditingWidget;
use crate::global::Global;
use crate::input_handler::InputHandler;
use crate::widget::Widget;
use getch_rs::Key;
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::Block;

pub struct GlobalWidget {
    editing_widget: EditingWidget,
    filename_widget: EditingWidget,
    save_requested: bool,
    save_realm: String,
}

impl GlobalWidget {
    pub fn new() -> Self {

        let mut filename_widget = EditingWidget::single_line();
        filename_widget.set_title("Name");

        Self {
            editing_widget: EditingWidget::multi_line(),
            filename_widget,
            save_requested: false,
            save_realm: String::new(),
        }
    }

    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([vertical])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    fn request_save_as(&mut self) {
        self.save_requested = true;
        self.filename_widget.set_focused(true);
        self.editing_widget.set_focused(false);
    }

    fn goto_top(&mut self) {
        self.save_requested = false;
        self.filename_widget.set_focused(false);
        self.editing_widget.set_focused(true);
    }
}

impl Widget for GlobalWidget {
    fn draw(&mut self, frame: &mut Frame, rect: Rect, global: &Global) {
        let [_, edit_rect, _] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(72),
                Constraint::Fill(1),
            ])
            .spacing(1)
            .split(rect)[..]
        else { unreachable!() };

        self.editing_widget.draw(frame, edit_rect, &global);

        if self.save_requested {
            let dialog = Block::bordered()
                .title("Save as...");

            let dialog_width = 32;
            let dialog_height = 9;

            let realm_lines = (self.save_realm.len() as f64 / dialog_width as f64 + 1.0) as u16;
            let mut constraints = vec![Constraint::Length(3)];
            for _ in 0..realm_lines {
                constraints.push(Constraint::Length(1));
            }
            constraints.push(Constraint::Length(1));
            constraints.push(Constraint::Length(1));

            let dialog_rect = Self::center(
                frame.area(),
                Constraint::Length(dialog_width + 2),
                Constraint::Length(dialog_height + realm_lines));
            let dialog_inner = dialog.inner(dialog_rect);

            let rects = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints)
                .split(dialog_inner);

            self.filename_widget.draw(frame, rects[0], &global);
            frame.render_widget(dialog, dialog_rect);


            let mut i = 1;
            self.save_realm
                .chars()
                .collect::<Vec<char>>()
                .chunks(dialog_inner.width as usize)
                .map(|chunk| Text::raw(chunk.iter().collect::<String>()).bold().red())
                .for_each(|text| {
                    frame.render_widget(text, rects[i]);
                    i += 1;
                });


            let submit_button = Text::raw("Save (enter)").italic().right_aligned();
            frame.render_widget(submit_button, *rects.last().unwrap());
        }
    }
}

impl InputHandler for GlobalWidget {
    fn handle(&mut self, c: &Key, global: &Global) -> bool {
        let mut key = c;

        if let Key::Char('\r') = key {
            key = &Key::Char('\n');
        }

        match key {
            Key::Esc if self.save_requested => {
                self.goto_top(); // TODO: Close file-saving window only
                self.filename_widget.set_content("");
            }
            Key::Esc => {
                return true;
            }
            Key::Char('\n') if self.save_requested => {

                match File::create_new(self.filename_widget.get_content())
                    .map(|mut file| file.write_all(self.editing_widget.get_content().as_bytes())){
                    Ok(Ok(_)) => {
                        self.goto_top(); // TODO: Close file-saving window only
                    }
                    Ok(Err(e)) => {
                        self.save_realm = e.to_string()
                    }
                    Err(e) => {
                        self.save_realm = e.to_string()
                    }
                };

                return false;
            }
            Key::Ctrl('s') => {
                self.request_save_as();
            }
            _ => {}
        }

        let shutdown = if self.save_requested {
            self.filename_widget.handle(key, global)
        } else {
            self.editing_widget.handle(key, global)
        };
        if shutdown {
            return true;
        }

        false
    }
}
