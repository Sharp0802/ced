use getch_rs::Key;
use ratatui::prelude::*;
use crate::editing_widget::EditingWidget;
use crate::global::Global;
use crate::input_handler::InputHandler;
use crate::widget::Widget;

pub struct GlobalWidget {
    editing_widget: EditingWidget,
}

impl GlobalWidget {
    pub fn new() -> Self {
        Self {
            editing_widget: EditingWidget::new(),
        }
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
    }
}

impl InputHandler for GlobalWidget {
    fn handle(&mut self, c: &Key, global: &Global) -> bool {
        if self.editing_widget.handle(c, global) {
            return true;
        }

        match c {
            Key::Esc => {
                return true;
            }
            Key::Ctrl('s') => {
            }
            _ => {}
        }

        false
    }
}
