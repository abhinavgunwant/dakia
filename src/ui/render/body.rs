use log::info;

use tui::{
    backend::Backend, style::{ Color, Style }, Frame,
    widgets::{ BorderType, Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction, Rect },
};

use crate::ui::{
    state::{
        UiState, UIElement, request_tabs::RequestTabs, kv_tab_state::KVTabState,
        body::{BodyContent, BodyUIElement},
    },
    widgets::{ text_input::TextInput, label::Label, select::Select },
    render::render_kv_tab,
};

pub fn render_body<B: Backend>(
    f: &mut Frame<B>, uistate: &mut UiState, rect: Rect
) {
    let body_content_rect = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .direction(Direction::Vertical)
        .split(rect);
    
    let body_top_rect = Layout::default()
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .direction(Direction::Horizontal)
        .split(body_content_rect[0]);

    match uistate.body().body_content() {
        BodyContent::FormData | BodyContent::FormURLEncoded => {
            //info!("Rendering kv tab");

            render_kv_tab(
                f,
                uistate.body().kv_tab_state(),
                uistate.body().kv_data(),
                body_content_rect[1],
            );
        }

        BodyContent::Text | BodyContent::Html | BodyContent::Xml => {
            let text_data = uistate.body().text_data();

            let text_multi_line = TextInput::default()
                .multi_line(true)
                .label(String::from(" Content "))
                .borders(Borders::ALL)
                .active(*uistate.body().active_body_element() == BodyUIElement::TextArea)
                .border_style(Style::default().fg(Color::White))
                .line_number(text_data.line_number())
                .cursor_pos(text_data.cursor_pos())
                .selecting(text_data.selecting())
                .sel_start_pos(text_data.sel_start_pos())
                .sel_end_pos(text_data.sel_end_pos())
                .text_vec(text_data.text_vec())
                .scroll_offset(text_data.scroll_offset());

            uistate.body_mut().text_data_mut().set_content_height(
                body_content_rect[1].height - 2
            );

            f.render_widget(text_multi_line, body_content_rect[1]);
        }

        _ => {}
    }

    let mut body_content_select = Select::default()
        .label(String::from(" Content Type "))
        .default_index(0)
        .disp_content_length(5)
        .style(Style::default().fg(Color::White))
        .scroll_offset(*uistate.body().body_content_scroll_offset())
        .active_style(Style::default().fg(Color::Yellow))
        .sel_index(*uistate.body().body_content_sel_index())
        .options(uistate.body().body_content_options());

    match uistate.body().active_body_element() {
        BodyUIElement::ContentType(opened) => {
            body_content_select = body_content_select.active(true).opened(*opened);
        }

        _ => {}
    }

    f.render_widget(body_content_select, body_top_rect[0]);
}

