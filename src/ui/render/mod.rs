pub mod body;

use tui::{
    backend::Backend, style::{ Color, Style }, Frame,
    widgets::{ BorderType, Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction, Rect },
};

use crate::ui::{
    state::{
        UiState, UIElement, request_tabs::RequestTabs, kv_tab_state::KVTabState,
        kv_data::KVData,
    },
    widgets::{ text_input::TextInput, label::Label },
    render::body::render_body,
};

/// Renders tab content
pub fn render_tab_content<B: Backend>(
    f: &mut Frame<B>, uistate: &mut UiState, rect: Rect
) {
    let mut tab_style = Style::default();

    if uistate.active_element() == &UIElement::RequestTabsElem {
        tab_style = tab_style.bg(Color::Reset).fg(Color::Yellow);
    }

    let tab_content_box = Block::default()
        .borders(Borders::ALL)
        .style(tab_style)
        .border_type(BorderType::Rounded);

    f.render_widget(tab_content_box, rect);

    let rect_inset = Rect::new(
        rect.x + 2,
        rect.y + 1,
        rect.width - 4,
        rect.height - 2
    );

    match uistate.active_request_tab() {
        RequestTabs::UrlParams => {
            //render_kv_tab(f, uistate, RequestTabs::UrlParams, rect);
            render_kv_tab(
                f,
                uistate.query_params_ui(),
                uistate.url_deconst().query_params(),
                rect
            );
        },
        RequestTabs::Authorization => {
            let content = Paragraph::new(String::from("Authorization"));
            f.render_widget(content, rect_inset);
        },
        RequestTabs::Headers => {
            //render_kv_tab(f, uistate, RequestTabs::Headers, rect);
            render_kv_tab(f,
                uistate.request_headers_ui(),
                uistate.request_headers(),
                rect
            );
        },
        RequestTabs::Body => {
            render_body(f, uistate, rect_inset);
        },
    }
}

pub fn render_kv_tab<B: Backend>(
    f: &mut Frame<B>,
    params: KVTabState,
    kv_data: &Vec<KVData>,
    //render_tab: RequestTabs,
    rect: Rect
) {
//    let params: KVTabState;
//    let kv_data;

//    match render_tab {
//        RequestTabs::UrlParams => {
//            params = uistate.query_params_ui();
//            kv_data = uistate.url_deconst().query_params().iter().enumerate();
//        },
//        RequestTabs::Headers => {
//            params = uistate.request_headers_ui();
//            kv_data = uistate.request_headers().iter().enumerate();
//        },
//        _ => { return; }
//    }

    let content_rect = Rect::new(
        rect.x + 2,
        rect.y + 1,
        rect.width - 4,
        3,
    );

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(10),
        ].as_ref())
        .split(content_rect);

    let param_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(content_chunks[0]);

    let param_actions_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(content_chunks[1]);

    for (i, param) in kv_data.iter().enumerate() {
        let mut name_rect = param_chunks[0];
        let mut value_rect = param_chunks[1];
        let mut add_param_chunk = param_actions_chunk[0];
        let mut remove_param_chunk = param_actions_chunk[1];
        let row_active: bool = params.active_row() == (i as u16);

        if i > 0 {
            name_rect.y += (3 * i) as u16;
            value_rect.y += (3 * i) as u16;
            add_param_chunk.y += (3 * i) as u16;
            remove_param_chunk.y += (3 * i) as u16;
        }

        let mut param_name_style = Style::default().fg(Color::Gray);

        if row_active && params.active_col() == 0 {
            param_name_style = param_name_style.fg(Color::Yellow);
        }

        let param_name = TextInput::default()
            .label(String::from(" Key "))
            .borders(Borders::ALL)
            .text(param.key())
            .border_style(param_name_style);

        f.render_widget(param_name, name_rect);

        let mut param_value_style = Style::default().fg(Color::Gray);

        if row_active && params.active_col() == 1 {
            param_value_style = param_value_style.fg(Color::Yellow);
        }

        let param_value = TextInput::default()
            .label(String::from(" Value "))
            .borders(Borders::ALL)
            .text(param.value())
            .border_style(param_value_style);

        f.render_widget(param_value, value_rect);

        let mut param_add_style = Style::default().fg(Color::White);

        if row_active && params.active_col() == 2 {
            param_add_style = param_add_style.fg(Color::Cyan);
        }

        let action_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let add_param = action_block.clone().style(param_add_style);

        f.render_widget(add_param, add_param_chunk);

        add_param_chunk.x += 2;
        add_param_chunk.width -= 2;
        add_param_chunk.y += 1;

        f.render_widget(
            Label::default().text("+").style(param_add_style),
            add_param_chunk
        );

        let mut param_remove_style = Style::default().fg(Color::White);

        if row_active && params.active_col() == 3 {
            param_remove_style = param_remove_style.fg(Color::Cyan);
        }

        let remove_param = action_block.style(param_remove_style);

        f.render_widget(remove_param, remove_param_chunk);

        remove_param_chunk.x += 2;
        remove_param_chunk.y += 1;

        f.render_widget(
            Label::default().text("-").style(param_remove_style),
            remove_param_chunk,
        );
    }
}

