use getch_rs::Key;
use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::global::Global;
use crate::input_field::Input;
use crate::input_handler::InputHandler;
use crate::widget::Widget;

pub struct EditingWidget {
    editing_field: Input,
}

impl EditingWidget {
    pub fn new() -> Self {
        Self {
            editing_field: Input::multi_line()
        }
    }
}

impl InputHandler for EditingWidget {
    fn handle(&mut self, c: &Key, _global: &Global) -> bool {
        self.editing_field.put_char(c);
        false
    }
}

impl Widget for EditingWidget {
    fn draw(&mut self, frame: &mut Frame, rect: Rect, global: &Global) {
        let p = Paragraph::new(self.editing_field.get_element())
            .block(Block::bordered()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .title(if global.current_file().len() == 0 { "*" } else { global.current_file() }))
            .wrap(Wrap { trim: true });

        frame.render_widget(p, rect);
    }
}
