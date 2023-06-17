use log::info;
use tui::{
    buffer::Buffer,
    layout::{ Alignment, Layout, Constraint, Direction, Rect },
    widgets::{ BorderType, Borders, Block, Widget, Paragraph },
    text::Span,
    style::{ Color, Style },
};

use crate::ui::{ string_chunks, string_chunks_to_spans, calc::scrollbar_pos };

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
    selecting: bool,
    sel_start_pos: (u16, u16),
    sel_end_pos: (u16, u16),
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
            selecting: false,
            sel_start_pos: (0, 0),
            sel_end_pos: (0, 0),
        }
    }
}

impl Widget for TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        // text_area_outer contains text_area and scrollbar
        let mut text_area_full = area.clone();

        text_area_full.y += 1;
        text_area_full.x += 2;
        text_area_full.width -= 4;
        text_area_full.height -= 2;

        let text_area_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(1),
            ].as_ref())
            .split(text_area_full);

        let text_area_height = text_area_full.height;

        let content_overflows = self.get_text_vec().len() as u16 > text_area_height;

        let text_area;

        if content_overflows {
            text_area = text_area_chunks[0];
        } else {
            text_area = text_area_full;
        }

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
            let mut text_end: usize;

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

                if self.is_selecting() {
                    let inner_area = Rect::new(
                        area.x + 2,
                        area.y + 1,
                        area.width - 4,
                        area.height - 2,
                    );

                    self.render_selection(inner_area, buf);
                }

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

            if content_overflows {
                self.render_scrollbar(text_area_chunks[1], buf);
            }
        } else {
            let mut text: String;

            match self.get_text() {
                Some (txt) => {
                    text = txt.clone();
                },

                None => {
                    text = String::default();
                },
            }

            let text_par: Paragraph;

            if self.is_active() {
                text.push('\u{2502}');

                text_par = Paragraph::new(
                    Span::styled(text, self.get_active_border_style())
                );
            } else {
                text_par = Paragraph::new(text)
                    .style(*self.get_border_style());
            }

            text_par.render(text_area, buf);
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

    pub fn is_multi_line(&self) -> bool { self.multi_line }
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

    pub fn is_selecting(&self) -> bool { self.selecting }
    pub fn selecting(mut self, selecting: bool) -> TextInput {
        self.selecting = selecting;
        self
    }

    pub fn get_sel_start_pos(&self) -> (u16, u16) { self.sel_start_pos.clone() }
    pub fn sel_start_pos(mut self, sel_start_pos: (u16, u16)) -> TextInput {
        self.sel_start_pos = sel_start_pos;
        self
    }

    pub fn get_sel_end_pos(&self) -> (u16, u16) { self.sel_end_pos.clone() }
    pub fn sel_end_pos(mut self, sel_end_pos: (u16, u16)) -> TextInput {
        self.sel_end_pos = sel_end_pos;
        self
    }

    /// Renders text selection whenever textarea is displayed
    fn render_selection(&self, area: Rect, buf: &mut Buffer) {
        let start_pos = self.get_sel_start_pos();
        let end_pos = self.get_sel_end_pos();

        let line_diff = (start_pos.0 as i32 - end_pos.0 as i32).abs();

        let sel_style = Style::default().fg(Color::White).bg(Color::DarkGray);

        match line_diff {
            0 => {
                let sel_block = Block::default().style(sel_style).borders(Borders::NONE);
                let x = area.x + start_pos.1;
                let y = area.y + start_pos.0;
                let width = end_pos.1 - start_pos.1 + 1;

                sel_block.render(Rect::new(x, y, width, 1), buf);
            }

            1 => {
                self.render_sel_multiline_ends(area, buf);
            }

            // 2 or more lines selected.
            _ => {
                let sel_block = Block::default().style(sel_style).borders(Borders::NONE);

                sel_block.render(Rect::new(
                    area.x,
                    area.y + start_pos.0 + 1,
                    area.width,
                    (line_diff - 1).abs() as u16,
                ), buf);

                self.render_sel_multiline_ends(area, buf);
            }
        }
    }

    fn render_sel_multiline_ends(&self, area: Rect, buf: &mut Buffer) {
        let start_pos = self.get_sel_start_pos();
        let end_pos = self.get_sel_end_pos();

        let sel_style = Style::default().fg(Color::White).bg(Color::DarkGray);

        let sel_block = Block::default().style(sel_style).borders(Borders::NONE);

        let x_pos = area.x + start_pos.1;

        let rect_start = Rect::new(
            x_pos,
            area.y + start_pos.0,
            area.width - start_pos.1,
            1
        );

        let rect_end = Rect::new(
            area.x,
            area.y + end_pos.0,
            end_pos.1 + 1,
            1
        );

        sel_block.clone().render(rect_start, buf);
        sel_block.render(rect_end, buf);
    }

    fn render_scrollbar(&self, area: Rect, buf: &mut Buffer) {
        let scrollbar_background = Block::default()
            .style(Style::default().bg(Color::DarkGray));

        let scrollbar_thumb = Block::default()
            .style(Style::default().bg(Color::White));

        let end_offset;
        let top_offset = self.get_scroll_offset();
        let total_content_len = self.get_text_vec().len() as u16;

        if top_offset + area.height < total_content_len {
            end_offset = top_offset + area.height;
        } else {
            end_offset = total_content_len;
        }

        let (height, y) = scrollbar_pos(
            area.height,
            end_offset,
            total_content_len,
        );

        scrollbar_background.render(area, buf);
        scrollbar_thumb.render(Rect::new(area.x, area.y + y, 1, height), buf);
    }
}

