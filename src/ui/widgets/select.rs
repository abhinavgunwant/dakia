use tui::{
    buffer::Buffer,
    layout::{ Alignment, Rect },
    widgets::{ BorderType, Borders, Block, Widget, Paragraph },
    text::Span,
    style::{ Color, Style },
};

use reqwest::Method;

#[derive(Clone, PartialEq, Eq)]
pub enum SelectVariant {
    Outline, Compact,
}

#[derive(Clone, PartialEq, Eq)]
pub struct SelectOptions<T> {
    name: String,
    value: T,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Select {
    label: String,
    options: Vec<String>,
    variant: SelectVariant,

    /// Index of the `options` vector that is selected by default
    default_index: Option<u8>,

    opened: bool,
    active: bool,
    style: Style,
    active_style: Style,
}

impl Default for SelectVariant {
    fn default() -> Self { Self::Outline }
}

impl Widget for Select {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        match self.variant {
            SelectVariant::Outline => {
                let style: Style;

                if *self.is_active() {
                    style = *self.get_active_style();
                } else {
                    style = *self.get_style();
                }

                let mut block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(style);

                if !self.get_label().is_empty() {
                    block = block.title(self.label.as_str());
                }

                match self.get_default_index() {
                    Some(indx) => {
                        if self.get_options().len() > *indx as usize {
                            let selected_text = Paragraph::new(
                                Span::styled(
                                    self.get_options()[*indx as usize].clone(),
                                    style
                                )
                            );
                            selected_text.render(area, buf);
                        }
                    }
                    None => {}
                }

                block.render(area, buf);
            }
            SelectVariant::Compact => {}
        }
    }
}

impl Select {
    fn get_label(&self) -> &String { &self.label }
    fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    fn get_options(&self) -> &Vec<String> { &self.options }
    fn options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    fn get_variant(&self) -> &SelectVariant { &self.variant }
    fn variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    fn get_default_index(&self) -> &Option<u8> { &self.default_index }
    fn default_index(mut self, indx: u8) -> Self {
        self.default_index = Some(indx);
        self
    }

    fn is_opened(&self) -> &bool { &self.opened }
    fn opened(mut self, opened: bool) -> Self { self.opened = opened; self }

    fn is_active(&self) -> &bool { &self.active }
    fn active(mut self, active: bool) -> Self { self.active = active; self }

    fn get_style(&self) -> &Style { &self.style }
    fn style(mut self, style: Style) -> Self { self.style = style; self }

    fn get_active_style(&self) -> &Style { &self.active_style }
    fn active_style(mut self, active_style: Style) -> Self {
        self.active_style = active_style; self
    }
}

