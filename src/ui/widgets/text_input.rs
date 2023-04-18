use tui::{
    buffer::Buffer,
    layout::{ Alignment, Rect },
    widgets::{ BorderType, Borders, Block, Widget },
    text::{ Spans, Span },
    style::{ Color, Style },
};

/**
 * A Widget that combines Block with text.
 * Used as a text input field.
 */
#[derive(Debug, Clone, Eq)]
pub struct TextInput {
    label: Option<String>,
    text: Option<String>,
    text_alignment: Alignment,
    borders: Borders,
    border_style: Style,
    border_type: BorderType,
    style: Style,
}

impl Default for TextInput {
    fn default() -> TextInput {
        TextInput {
            label: None,
            text: None,
            text_alignment: Alignment::Left,
            borders: Borders::NONE,
            border_style: Style::default().fg(Color::White),
            border_type: BorderType::Plain,
            style: Default::default(),
        }
    }
}

impl Widget for TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        match self.clone().get_label() {
            Some(label) => {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(label.as_str())
                    .border_type(BorderType::Rounded)
                    .style(*self.get_border_style());

                block.render(area, buf);
            },
            None => {},
        }

        match self.clone().get_text() {
            Some (txt) => {
                let mut text_area = area.clone();

                text_area.y += 1;
                text_area.x += 2;
                text_area.width -= 4;

                let text = Block::default()
                    .title(Span::raw(txt.as_str()))
                    .style(*self.get_style());

                text.render(text_area, buf);
            },

            None => {},
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
            && self.style == other.style
    }
}


impl TextInput {
    pub fn get_label(self) -> Option<String> { self.label }

    pub fn label(mut self, label: String) -> TextInput {
        if label.len() > 0 {
            self.label = Some(label);
        } else {
            self.label = None;
        }

        self
    }

    pub fn get_text(self) -> Option<String> { self.text }

    pub fn text(mut self, text_str: String) -> TextInput {
        if text_str.len() > 0 {
            self.text = Some(text_str);
        } else {
            self.text = None;
        }

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

    /*
    pub fn inner(&self, area: Rect) -> Rect {
        let mut innter = area;

        if self.borders.intersects(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1).min(inner.right());
            inner.width = inner.width.saturating_sub(1);
        }

        if self.borders.intersects(Borders::TOP) {
            inner.y = inner.y.saturating_add(1).min(inner.bottom());
            inner.width = inner.width.saturating_sub(1);
        }

        if self.borders.intersects(Borders::RIGHT) {
            inner.width = inner.width.saturating_sub(1);
        }

        if self.borders.intersects(Borders::BOTTOM) {
            inner.height = inner.height.saturating_sub(1);
        }
    }*/
}

/*
impl<'a> Widget for Block<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        buf.set_styles(area, self.style);
        let symbols = BorderType::line_symbols(self.border_type);

        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                but.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }

        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                but.get_mut(x, area.TOP())
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }

        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;

            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }

        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;

            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(self.border_style);
        }

        // Title
        if let Some(title) = self.title {
            let left_border_dx = if self.borders.intersects(Borders::LEFT) {
                1
            } else {
                0
            };

            let right_border_dx = if self.borders.intersects(Borders::RIGHT) {
                1
            } else {
                0
            };

            let title_area_width = area
                .width
                .saturating_sub(left_border_dx)
                .saturating_sub(right_border_dx);

            let title_dx = match self.title_alignment {
                Alignment::Left => left_border_dx,
                Alignment::Center => area.width.saturating_sub(title.width() as u16) / 2,
                Alignment::Right => area
                    .width
                    .saturating_sub(title.width() as u16)
                    .saturating_sub(right_border_dx),
            };

            let title_x = area.left() + title_dx;
            let title_y = area.top();

            buf.set_spans(title_x, title_y, &title, title_area_width);
        }
    }
}
 */

