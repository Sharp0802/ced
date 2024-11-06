use crate::global::Global;
use crate::input_handler::InputHandler;
use crate::widget::Widget;
use getch_rs::Key;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::Frame;
use std::cmp::min;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct EditingWidget {
    single_line: bool,
    title: String,
    content: String,
    cursor: usize,
    scroll: usize,
    focused: bool,
}

impl Widget for EditingWidget {
    fn draw(&mut self, frame: &mut Frame, rect: Rect, _global: &Global) {

        let block = Block::bordered()
            .title(self.title.clone())
            .border_type(if self.single_line { BorderType::Rounded } else { BorderType::Plain });

        let block_inner = block.inner(rect);

        let mut block_rect = rect;
        if self.single_line {
            block_rect.height = 3;
        }
        frame.render_widget(block, block_rect);

        let [_, mut content_rect, scrollbar_rect] = Layout::horizontal(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).split(block_inner)[..] else {
            unreachable!()
        };

        if self.single_line {
            content_rect.height = 1;
            content_rect.width += 1;
        }

        let p = Paragraph::new(self.get_text())
            .scroll((self.scroll as u16, 0))
            .wrap(Wrap { trim: false });

        frame.render_widget(p, content_rect);

        // scrollbar shouldn't be drawn when single-line mode
        if self.single_line {
            return;
        }

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("┃")
            .track_symbol(Some("│"))
            .begin_symbol(None)
            .end_symbol(None);
        let mut scrollbar_state = ScrollbarState::new(self.total_line()).position(self.scroll);

        frame.render_stateful_widget(scrollbar, scrollbar_rect, &mut scrollbar_state);
    }
}



impl EditingWidget {
    pub fn single_line() -> Self {
        Self {
            single_line: true,
            title: String::new(),
            content: String::new(),
            cursor: 0,
            scroll: 0,
            focused: true,
        }
    }

    pub fn multi_line() -> Self {
        Self {
            single_line: false,
            title: String::new(),
            content: String::new(),
            cursor: 0,
            scroll: 0,
            focused: true,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn get_focused(&self) -> bool {
        self.focused
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.cursor = 0;
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn next_char(&mut self) {
        if self.cursor >= self.content.len() {
            return;
        }

        self.cursor += 1;
        while self.content.bytes().nth(self.cursor).unwrap_or(0) & 0xC0 == 0x80 {
            self.cursor += 1;
        }
    }

    fn prev_char(&mut self) {
        if self.cursor <= 0 {
            return;
        }

        self.cursor -= 1;
        while self.content.bytes().nth(self.cursor).unwrap() & 0xC0 == 0x80 {
            self.cursor -= 1;
        }
    }

    fn climb_line(&mut self) {
        if self.cursor <= 1 {
            return;
        }

        let mut width = 1;
        self.cursor -= 1;
        while self.content.bytes().nth(self.cursor).unwrap_or(0) != 0x0A && self.cursor > 0 {
            self.cursor -= 1;
            width += 1;
        }

        if self.cursor <= 1 {
            return;
        }

        let mut upper_width = 1;
        self.cursor -= 1;
        while self.content.bytes().nth(self.cursor).unwrap_or(0) != 0x0A && self.cursor > 0 {
            self.cursor -= 1;
            upper_width += 1;
        }

        let mut debt: i32 = 0;
        if self.content.bytes().nth(self.cursor).unwrap() != 0x0A {
            debt -= 1;
        }

        self.cursor += (min(upper_width, width) + debt) as usize;
    }

    fn descend_line(&mut self) {

        let mut width = 1;
        let mut tmp_cursor = self.cursor;

        while {
            if tmp_cursor > 0 {
                tmp_cursor -= 1;
            }
            width += 1;

            self.content.bytes().nth(tmp_cursor).unwrap_or(0) != 0x0A && tmp_cursor > 0
        } {};

        let mut content_length = self.content.len();
        if content_length > 0 {
            content_length -= 1;
        }
        if self.cursor == content_length {
            return;
        }


        while self.content.bytes().nth(self.cursor).unwrap_or(0) != 0x0A && self.cursor < content_length {
            self.cursor += 1;
        }
        if self.content.bytes().nth(self.cursor).unwrap_or(0) != 0x0A && self.cursor > 0 {
            self.cursor -= 1;
        }

        self.cursor += width;
        if self.cursor >= self.content.len() {
            self.cursor = self.content.len() - 1;
        }
    }

    fn lines_any(input: &str) -> Vec<&str> {
        let mut lines: Vec<&str> = input.lines().collect();
        if input.ends_with('\n') {
            lines.push("");
        }
        lines
    }

    fn total_line(&self) -> usize {
        self.content.bytes().filter(|&b| b == 0x0A).count() + 1
    }

    fn line_number_str(mut num: usize, cur: usize) -> String {
        let mut size = 0;
        while num > 0 {
            num /= 10;
            size += 1;
        }

        format!("{1:0$}", size, cur)
    }

    fn line_number_txt<'a>(num: usize, cur: usize) -> Span<'a> {
        Span::styled(
            Self::line_number_str(num, cur) + "  ",
            Style::default().dim())
    }

    fn get_text(&self) -> Text {

        let lines = Self::lines_any(self.content.as_str());

        let mut current_line: usize = 1;
        let total_line = self.content.bytes().filter(|b| *b == 0x0A).count();

        let mid_style = if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() % 1000 > 500 || !self.focused {
            Style::default()
        } else {
            Style::default().reversed()
        };

        let element: Line = if self.single_line {
            Line::default()
        } else {
            Line::from(Self::line_number_txt(total_line, current_line))
        };

        if self.content.len() == 0 {
            Text::from(element + Span::styled(" ", mid_style))
        } else {
            let mut offset: usize = 0;
            Text::from(lines.into_iter().map(|line| {
                let mut element = element.clone();

                let new_offset = offset + line.len() + 1;
                if offset <= self.cursor && self.cursor < new_offset - 1 {

                    // split line horizontally by cursor position
                    let (left, right) = line
                        .split_at_checked(self.cursor - offset)
                        .unwrap();

                    let at = right.chars().nth(0).unwrap_or(' ').len_utf8();

                    // separate cursor character from right part of line
                    let (mid, right) = right
                        .split_at_checked(at)
                        .unwrap_or((" ", ""));

                    element += Span::styled(left, Style::default());
                    element += Span::styled(mid, mid_style);
                    element += Span::styled(right, Style::default());
                } else if self.cursor == new_offset - 1 {

                    // if last character is line-feed, display linebreak symbol
                    let mid = if self.content.bytes().nth(self.cursor).unwrap_or(0) == 0x0A { "↵" } else { " " };

                    element += Span::styled(line, Style::default());
                    element += Span::styled(mid, mid_style);
                } else {
                    element += Span::styled(line, Style::default());
                }

                *(&mut offset) = new_offset;
                current_line += 1;

                element
            }).collect::<Vec<Line>>())
        }
    }
}


impl InputHandler for EditingWidget {
    fn handle(&mut self, c: &Key, _global: &Global) -> bool {
        let mut key = c.clone();

        if let Key::Char('\t') = key {
            self.handle(&Key::Char(' '), _global);
            self.handle(&Key::Char(' '), _global);
            self.handle(&Key::Char(' '), _global);
            self.handle(&Key::Char(' '), _global);
            return false;
        }

        if let Key::Char('\r') = key {
            key = Key::Char('\n');
        }

        if self.single_line {
            if let Key::Char('\n') = key {
                return false;
            }
        }

        match key {
            Key::Char(char) => {
                self.content.insert(self.cursor, char);
                self.cursor += char.len_utf8();
            }

            Key::Backspace => {
                if self.cursor > 0 {
                    self.prev_char();
                    self.content.remove(self.cursor);
                }
            }
            Key::Delete => {
                if self.cursor < self.content.len() {
                    self.content.remove(self.cursor);
                }
            }

            Key::Left => self.prev_char(),
            Key::Right => self.next_char(),
            Key::Up => self.climb_line(),
            Key::Down => self.descend_line(),

            Key::PageDown => {
                if self.scroll + 1 >= self.total_line() {
                    return false;
                }

                self.scroll += 1;
            }
            Key::PageUp => {
                if self.scroll <= 0 {
                    return false;
                }

                self.scroll -= 1;
            }
            Key::Home => {
                self.scroll = 0;
            }
            Key::End => {
                self.scroll = self.total_line() - 1;
            }

            _ => {}
        }

        false
    }
}
