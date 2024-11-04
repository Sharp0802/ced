use ratatui::prelude::*;
use std::cmp::min;
use std::time::{SystemTime, UNIX_EPOCH};
use getch_rs::Key;

pub struct Input {
    single_line: bool,
    content: String,
    cursor: usize
}

impl Input {
    pub fn single_line() -> Self {
        Self {
            single_line: true,
            content: String::new(),
            cursor: 0
        }
    }

    pub fn multi_line() -> Self {
        Self {
            single_line: false,
            content: String::new(),
            cursor: 0
        }
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
        while self.content.bytes().nth(tmp_cursor).unwrap_or(0) != 0x0A && tmp_cursor > 0 {
            tmp_cursor -= 1;
            width += 1;
        }

        while self.content.bytes().nth(self.cursor).unwrap_or(0) != 0x0A && self.cursor < self.content.len() - 1 {
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

    pub fn put_char(&mut self, mut c: &Key) {
        if let Key::Char('\t') = c {
            self.put_char(&Key::Char(' '));
            self.put_char(&Key::Char(' '));
            self.put_char(&Key::Char(' '));
            self.put_char(&Key::Char(' '));
            return;
        }

        if let Key::Char('\r') = c {
            c = &Key::Char('\n');
        }

        if self.single_line {
            if let Key::Char('\n') = c {
                return;
            }
        }

        match c {
            Key::Char(char) => {
                self.content.insert(self.cursor, *char);
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

            _ => {}
        }
    }

    pub fn get_element(&self) -> Text {
        let lines = Self::lines_any(self.content.as_str());

        let mut current_line: usize = 1;
        let total_line = self.content.bytes().filter(|b| *b == 0x0A).count();

        let mid_style = if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() % 1000 > 500 {
            Style::default()
        } else {
            Style::default().reversed()
        };

        if self.content.len() == 0 {
            Text::from(Self::line_number_txt(total_line, current_line) +
                Span::styled(" ", mid_style))
        } else {
            let mut offset: usize = 0;
            Text::from(lines.into_iter().map(|line| {
                let element;

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

                    element = Self::line_number_txt(total_line, current_line) +
                        Span::styled(left, Style::default()) +
                        Span::styled(mid, mid_style) +
                        Span::styled(right, Style::default());
                } else if self.cursor == new_offset - 1 {

                    // if last character is line-feed, display linebreak symbol
                    let mid = if self.content.bytes().nth(self.cursor).unwrap_or(0) == 0x0A { "↵" } else { " " };

                    element = Self::line_number_txt(total_line, current_line) +
                        Span::styled(line, Style::default()) +
                        Span::styled(mid, mid_style);
                } else {
                    element = Self::line_number_txt(total_line, current_line) +
                        Span::styled(line, Style::default());
                }

                *(&mut offset) = new_offset;
                current_line += 1;

                element
            }).collect::<Vec<Line>>())
        }
    }
}
