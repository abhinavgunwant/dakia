use tui::{
    buffer::Buffer,
    layout::{ Alignment, Rect },
    widgets::{ BorderType, Borders, Block, Widget, Paragraph, Clear },
    text::Span,
    style::{ Color, Style },
};

use crate::ui::calc::scrollbar_pos;

#[derive(Clone, PartialEq, Eq)]
pub enum SelectVariant {
    Outline, Compact,
}

#[derive(Clone, PartialEq, Eq)]
pub struct SelectOptions<T> {
    name: String,
    value: T,
}

impl Default for SelectVariant {
    fn default() -> Self { Self::Outline }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Select {
    label: String,
    options: Vec<String>,
    variant: SelectVariant,

    /// Index of the `options` vector that is selected by default
    default_index: u8,
    display_index: u8,

    /// Represented the element that is being highlighted when the this widget
    /// is in the `opened` state.
    sel_index: u8,

    scroll_offset: u8,

    /// The length of content displayed at once
    disp_content_length: u8,

    opened: bool,
    active: bool,
    style: Style,
    active_style: Style,
}

impl Default for Select {
    fn default() -> Self {
        Self {
            label: String::default(),
            options: vec![],
            variant: SelectVariant::default(),

            /// Index of the `options` vector that is selected by default
            default_index: 0,
            display_index: 0,

            /// Represented the element that is being highlighted when the this widget
            /// is in the `opened` state.
            sel_index: 0,

            scroll_offset: 0,

            /// The length of content displayed at once
            disp_content_length: 0,

            opened: false,
            active: false,
            style: Style::default().fg(Color::White),
            active_style: Style::default().fg(Color::Yellow),
        }
    }
}

impl Widget for Select {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        match self.variant {
            SelectVariant::Outline => {
                let style: Style;
                let show_scrollbar: bool = self.get_options().len() as u16 > *self.get_disp_content_length() as u16;

                if *self.is_active() {
                    style = *self.get_active_style();
                } else {
                    style = *self.get_style();
                }

                // render borders
                let mut block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(style);

                if !self.get_label().is_empty() {
                    block = block.title(self.label.as_str());
                }

                block.render(area, buf);

                let mut text_rect = Rect::new(
                    area.x + 2,
                    area.y + 1,
                    area.width,
                    1,
                );

                // render selected text
                if self.get_options().len() > *self.get_sel_index() as usize {
                    let text = self.get_options()[
                        *self.get_sel_index() as usize
                    ].clone();

                    let selected_text = Paragraph::new(
                        Span::styled(text, style)
                    );

                    selected_text.render(text_rect, buf);
                }

                // render popup if active and opened
                if *self.is_opened() && *self.is_active() {
                    // Clear the area under the popup
                    let popup_rect = Rect::new(
                        area.x,
                        area.y + 2,
                        area.width,
                        6
                    );

                    let clear = Clear;
                    clear.render(popup_rect, buf);

                    // render the popup items
                    let mut popup_element_rect;

                    if show_scrollbar {
                        popup_element_rect = Rect::new(
                            popup_rect.x + 1,
                            popup_rect.y - 1,
                            popup_rect.width - 4,
                            1,
                        );
                    } else {
                        popup_element_rect = Rect::new(
                            popup_rect.x + 1,
                            popup_rect.y - 1,
                            popup_rect.width - 2,
                            1,
                        );
                    }

                    let so = *self.get_scroll_offset() as usize;
                    let mut d =  *self.get_disp_content_length() as usize;

                    if self.get_options().len() < d {
                        d = self.get_options().len();
                    }

                    let items_to_display = &self.get_options()[so..(so+d)];
                    let highlight_item_style = Style::default()
                        .bg(Color::Yellow).fg(Color::Black);
                    let default_item_style = Style::default()
                        .bg(Color::Reset).fg(Color::White);

                    for (i, opt) in items_to_display.iter().enumerate() {
                        let paragraph: Paragraph;

                        popup_element_rect.y += 1 as u16;

                        if (so + i) as u8 == *self.get_sel_index() {
                            let highlight_block = Block::default()
                                .style(highlight_item_style);

                            highlight_block.render(popup_element_rect, buf);

                            paragraph = Paragraph::new(
                                Span::styled(opt, highlight_item_style)
                            );
                        } else {
                            paragraph = Paragraph::new(
                                Span::styled(opt, default_item_style)
                            );
                        }

                        let popup_element_par_rect = Rect::new(
                            popup_element_rect.x + 1,
                            popup_element_rect.y,
                            popup_element_rect.width,
                            popup_element_rect.height,
                        );

                        paragraph.render(popup_element_par_rect, buf);
                    }

                    if show_scrollbar {
                        let opt_length = *self.get_disp_content_length() as u16;
                        let (thumb_height, thumb_pos) = scrollbar_pos(
                            opt_length,
                            opt_length + *self.get_scroll_offset() as u16,
                            self.get_options().len() as u16,
                        );

                        let scrollbar_rect = Rect::new(
                            popup_rect.x + popup_rect.width - 2,
                            popup_rect.y,
                            1,
                            popup_rect.height - 1,
                        );

                        let scrollbar_thumb_rect = Rect::new(
                            scrollbar_rect.x,
                            scrollbar_rect.y + thumb_pos,
                            1,
                            thumb_height,
                        );

                        let scrollbar_track = Block::default()
                            .style(Style::default().bg(Color::DarkGray))
                            .borders(Borders::NONE);

                        scrollbar_track.render(scrollbar_rect, buf);

                        let scrollbar_thumb = Block::default()
                            .style(Style::default().bg(Color::Gray))
                            .borders(Borders::NONE);

                        scrollbar_thumb.render(scrollbar_thumb_rect, buf);
                    }

                    let popup_block = Block::default()
                        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Yellow));

                    popup_block.render(popup_rect, buf);

                    let triangle_up = Paragraph::new(
                        Span::styled(String::from("\u{eb71}"), style)
                    );

                    text_rect.x += text_rect.width - 5;
                    text_rect.width = 1;
                    triangle_up.render(text_rect, buf);
                } else {
                    let triangle_down = Paragraph::new(
                        Span::styled(String::from("\u{eb6e}"), style)
                    );

                    text_rect.x += text_rect.width - 5;
                    text_rect.width = 1;
                    triangle_down.render(text_rect, buf);
                }
            }

            SelectVariant::Compact => {}
        }
    }
}

impl Select {
    pub fn get_label(&self) -> &String { &self.label }
    pub fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn get_options(&self) -> &Vec<String> { &self.options }
    pub fn options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn get_variant(&self) -> &SelectVariant { &self.variant }
    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn get_default_index(&self) -> &u8 { &self.default_index }
    pub fn default_index(mut self, indx: u8) -> Self {
        self.default_index = indx;
        self
    }

    pub fn get_sel_index(&self) -> &u8 { &self.sel_index }
    pub fn sel_index(mut self, indx: u8) -> Self {
        self.sel_index = indx;
        self
    }

    pub fn get_scroll_offset(&self) -> &u8 { &self.scroll_offset }
    pub fn scroll_offset(mut self, offset: u8) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn get_disp_content_length(&self) -> &u8 { &self.disp_content_length }
    pub fn disp_content_length(mut self, content_length: u8) -> Self {
        self.disp_content_length = content_length;
        self
    }

    pub fn is_opened(&self) -> &bool { &self.opened }
    pub fn opened(mut self, opened: bool) -> Self { self.opened = opened; self }

    pub fn is_active(&self) -> &bool { &self.active }
    pub fn active(mut self, active: bool) -> Self { self.active = active; self }

    pub fn get_style(&self) -> &Style { &self.style }
    pub fn style(mut self, style: Style) -> Self { self.style = style; self }

    pub fn get_active_style(&self) -> &Style { &self.active_style }
    pub fn active_style(mut self, active_style: Style) -> Self {
        self.active_style = active_style; self
    }
}

