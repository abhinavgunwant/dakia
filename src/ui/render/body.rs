use tui::{
    backend::Backend, style::{ Color, Style }, Frame,
    widgets::{ BorderType, Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction, Rect },
};

use crate::ui::{
    state::{
        UiState, UIElement, request_tabs::RequestTabs, kv_tab_state::KVTabState,
        body::BodyContent,
    },
    widgets::{ text_input::TextInput, label::Label, select::Select },
};

fn body_content_options() -> Vec<String> {
    let mut v: Vec<String> = vec![];

    for item in BodyContent::iter() {
        v.push(item.to_string());
    }

    v
}

pub fn render_body<B: Backend>(
    f: &mut Frame<B>, uistate: &mut UiState, rect: Rect
) {
    let body_content_rect = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .direction(Direction::Vertical)
        .split(rect);
    
    let body_top_rect = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(Direction::Horizontal)
        .split(body_content_rect[0]);

    let select = Select::default()
        .label(String::from("Content type"))
        .default_index(0)
        .active(true)
        .opened(true)
        .options(body_content_options());
    f.render_widget(select, body_top_rect[0]);
}

