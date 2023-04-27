use tui::{
    buffer::Buffer,
    layout::{ Alignment, Rect },
    widgets::{ BorderType, Borders, Block, Widget, Paragraph },
    text::Span,
    style::{ Color, Style },
};


/// A Widget that combines Block with text.
/// Used as a text input field.
#[derive(Debug, Clone, Eq)]
pub struct TextInput {
    label: Option<String>,
    text: Option<String>,
    text_alignment: Alignment,
    borders: Borders,
    border_style: Style,
    active_border_style: Style,
    border_type: BorderType,
    style: Style,
    active: bool,
}

impl Default for TextInput {
    fn default() -> TextInput {
        TextInput {
            label: None,
            text: None,
            text_alignment: Alignment::Left,
            borders: Borders::NONE,
            border_style: Style::default(),
            active_border_style: Style::default(),
            border_type: BorderType::Plain,
            style: Default::default(),
            active: false,
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

        match self.get_label() {
            Some(label) => {
                let mut block = Block::default()
                    .borders(Borders::ALL)
                    .title(label.as_str())
                    .border_type(BorderType::Rounded);

                if self.is_active() {
                    block = block.style(self.get_active_border_style());
                } else {
                    block = block.style(*self.get_border_style());
                }

                block.render(area, buf);
            },
            None => {},
        }

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
                    let txt = String::from('\u{2502}');

                    let text = Paragraph::new(
                        Span::styled(txt, self.get_active_border_style())
                    );
                    text.render(text_area, buf);
                }
            },
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
}

