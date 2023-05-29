use tui::{
    buffer::Buffer,
    layout::{ Alignment, Rect },
    widgets::{ BorderType, Borders, Block, Widget, Paragraph },
    text::Span,
    style::{ Color, Style },
};

use crate::ui::{ string_chunks, string_chunks_to_spans };

/// A Widget that combines Block with text.
/// Used as a text input field.
#[derive(Debug, Clone, Eq)]
pub struct TextInput {
    label: Option<String>,
    text: Option<String>,
    text_vec: Vec<String>,
    text_alignment: Alignment,
    borders: Borders,
    border_style: Style,
    active_border_style: Style,
    border_type: BorderType,
    style: Style,
    active: bool,
    multi_line: bool,
    cursor_pos: u16,
    line_number: u16,
    scroll_offset: u16,
}

impl Default for TextInput {
    fn default() -> TextInput {
        TextInput {
            label: None,
            text: None,
            text_vec: vec![],
            text_alignment: Alignment::Left,
            borders: Borders::NONE,
            border_style: Style::default(),
            active_border_style: Style::default(),
            border_type: BorderType::Plain,
            style: Default::default(),
            active: false,
            multi_line: false,
            cursor_pos: 0,
            line_number: 0,
            scroll_offset: 0,
        }
    }
}

impl Widget for TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        let mut text_area = area.clone();

        text_area.y += 1;
        text_area.x += 2;
        text_area.width -= 4;

        // Block
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        match self.get_label() {
            Some(label) => { block = block.title(label.as_str()); }
            None => {}
        }

        if self.is_active() {
            block = block.style(self.get_active_border_style());
        } else {
            block = block.style(*self.get_border_style());
        }

        block.render(area, buf);

        // Text
        if self.is_multi_line() {
            let mut lines: Vec<String> = vec![];
            let mut text_start: usize = 0;
            let mut text_end: usize = self.get_text_vec().len();

            for line in self.get_text_vec() {
                if line.len() > text_area.width as usize {
                    let mut line_divided = string_chunks(line, text_area.width as usize);

                    lines.append(&mut line_divided);
                } else {
                    lines.push(line.clone());
                }
            }

            text_end = lines.len();

            if text_end > text_area.height as usize {
                text_start = self.get_scroll_offset() as usize;
                text_end = text_start + text_area.height as usize;
            }

            let text = Paragraph::new(
                string_chunks_to_spans(&lines[text_start..text_end])
            );
            text.render(text_area, buf);

            if self.is_active() {
                let cursor_block = Block::default()
                    .style(Style::default().fg(Color::Black).bg(Color::Yellow))
                    .borders(Borders::NONE);

                cursor_block.render(
                    Rect::new(
                        text_area.x + self.cursor_pos,
                        text_area.y + self.get_line_number()
                            - self.get_scroll_offset(),
                        1,
                        1
                    ),
                    buf
                );
            }
        } else {
            match self.get_text() {
                Some (txt) => {
                    if self.is_active() {
                        let mut txt_ = txt.clone();
                        txt_.push('\u{2502}');

                        let text = Paragraph::new(
                            Span::styled(txt_, self.get_active_border_style())
                        );
                        text.render(text_area, buf);
                    } else {
                        let text = Paragraph::new(txt.clone())
                            .style(*self.get_border_style());
                        text.render(text_area, buf);
                    }
                },

                None => {
                    if self.is_active() {
                        if !self.is_multi_line() {
                            let txt = String::from('\u{2502}');

                            let text = Paragraph::new(
                                Span::styled(txt, self.get_active_border_style())
                            );

                            text.render(text_area, buf);
                        } else {
                            let cursor_block = Block::default()
                                .style(
                                    Style::default()
                                        .fg(Color::Black)
                                        .bg(Color::Yellow)
                                )
                                .borders(Borders::NONE);

                            cursor_block.render(
                                Rect::new(text_area.x, text_area.y, 1, 1),
                                buf,
                            );
                        }
                    }
                },
            }
        }
    }
}

impl PartialEq for TextInput {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.text == other.text
            && self.text_alignment == other.text_alignment
            && self.borders == other.borders
            && self.border_style == other.border_style
            && self.border_type == other.border_type
            && self.active_border_style == other.active_border_style
            && self.style == other.style
    }
}


impl TextInput {
    pub fn get_label(&self) -> &Option<String> { &self.label }

    pub fn label(mut self, label: String) -> TextInput {
        if label.len() > 0 {
            self.label = Some(label);
        } else {
            self.label = None;
        }

        self
    }

    pub fn get_text(&self) -> &Option<String> { &self.text }

    pub fn text(mut self, text_str: String) -> TextInput {
        if text_str.len() > 0 {
            self.text = Some(text_str);
        } else {
            self.text = None;
        }

        self
    }

    pub fn get_text_vec(&self) -> &Vec<String> { &self.text_vec }
    pub fn text_vec(mut self, text_vec: Vec<String>) -> TextInput {
        self.text_vec = text_vec;
        self
    }

    pub fn get_text_alignment(self) -> Alignment { self.text_alignment }

    pub fn text_alignment(mut self, alignment: Alignment) -> TextInput {
        self.text_alignment = alignment;
        self
    }

    pub fn get_border_style(&self) -> &Style { &self.border_style }
    pub fn border_style(mut self, style: Style) -> TextInput {
        self.border_style = style;
        self
    }

    pub fn get_active_border_style(&self) -> Style { self.active_border_style }
    pub fn active_border_style(mut self, style: Style) -> TextInput {
        self.active_border_style = style;
        self
    }

    pub fn get_style(&self) -> &Style { &self.style }
    pub fn style(mut self, style: Style) -> TextInput {
        self.style = style;
        self
    }

    pub fn get_borders(self) -> Borders { self.borders }

    pub fn borders(mut self, borders: Borders) -> TextInput {
        self.borders = borders;
        self
    }

    pub fn get_border_type(self) -> BorderType { self.border_type }

    pub fn border_type(mut self, border_type: BorderType) -> TextInput {
        self.border_type = border_type;
        self
    }

    pub fn is_active(&self) -> bool { self.active }
    pub fn active(mut self, active: bool) -> TextInput {
        self.active = active;
        self
    }

    pub fn is_multi_line(&self) -> bool { self.active }
    pub fn multi_line(mut self, multi_line: bool) -> TextInput {
        self.multi_line = multi_line;
        self
    }
    
    pub fn get_cursor_pos(&self) -> u16 { self.cursor_pos.clone() }
    pub fn cursor_pos(mut self, cursor_pos: u16) -> TextInput {
        self.cursor_pos = cursor_pos;
        self
    }

    pub fn get_line_number(&self) -> u16 { self.line_number.clone() }
    pub fn line_number(mut self, line_number: u16) -> TextInput {
        self.line_number = line_number;
        self
    }

    pub fn get_scroll_offset(&self) -> u16 { self.scroll_offset.clone() }
    pub fn scroll_offset(mut self, scroll_offset: u16) -> TextInput {
        self.scroll_offset = scroll_offset;
        self
    }
}

