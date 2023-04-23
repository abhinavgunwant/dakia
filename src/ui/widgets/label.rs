use tui::{
    layout::Rect,
    buffer::Buffer,
    style::Style,
    widgets::Widget,
};

#[derive(Default)]
pub struct Label<'a> {
    style: Style,
    text: &'a str,
    margin_top: u16,
    margin_right: u16,
    margin_bottom: u16,
    margin_left: u16,
}

impl<'a> Label<'a> {
    pub fn text(mut self, text: &'a str) -> Label<'a> {
        self.text = text;
        self
    }

    pub fn margin_top(mut self, margin_top: u16) -> Label<'a> {
        self.margin_top = margin_top;
        self
    }

    pub fn margin_right(mut self, margin_right: u16) -> Label<'a> {
        self.margin_right = margin_right;
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: u16) -> Label<'a> {
        self.margin_bottom = margin_bottom;
        self
    }

    pub fn margin_left(mut self, margin_left: u16) -> Label<'a> {
        self.margin_left = margin_left;
        self
    }

    pub fn style(mut self, style: Style) -> Label<'a> {
        self.style = style;
        self
    }

    pub fn margin(
        mut self,
        top: Option<u16>,
        right: Option<u16>,
        bottom: Option<u16>,
        left: Option<u16>
    ) -> Label<'a> {
        match top {
            Some(top_val) => { self.margin_top = top_val; },
            None => {}
        }

        match right {
            Some(right_val) => { self.margin_right = right_val; },
            None => {}
        }

        match bottom {
            Some(bottom_val) => { self.margin_bottom = bottom_val; },
            None => {}
        }

        match left {
            Some(left_val) => { self.margin_left = left_val; },
            None => {}
        }

        self
    }
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            area.left() + self.margin_left,
            area.top() + self.margin_top,
            self.text,
            Style::default()
        );
    }
}

