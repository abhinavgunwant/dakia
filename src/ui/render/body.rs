use tui::{
    backend::Backend, style::{ Color, Style }, Frame,
    widgets::{ BorderType, Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction, Rect },
};

use crate::ui::{
    state::{
        UiState, UIElement, request_tabs::RequestTabs, kv_tab_state::KVTabState,
    },
    widgets::{ text_input::TextInput, label::Label },
};

pub fn render_body<B: Backend>(
    f: &mut Frame<B>, uistate: &mut UiState, rect: Rect
) {
    let content = Paragraph::new(String::from("Body"));
    f.render_widget(content, rect);
}

