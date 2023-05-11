use super::{kv_data::KVData, kv_tab_state::KVTabState};

#[derive(Clone)]
pub enum BodyContent {
    NONE,
    FormData(KVData),
    FormURLEncoded(KVData),
    Raw(RawBodyContentType),
}

/// To help with syntax highlighting later-on!
#[derive(Clone)]
pub enum RawBodyContentType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

#[derive(Clone)]
pub enum BodyCursorPosition {
    NONE,
    TabState(KVTabState),
    RawCursor(u16, u8),
}

#[derive(Clone, Default)]
pub struct Body {
    body_content: BodyContent,
    cursor_position: BodyCursorPosition,
}

impl Default for BodyContent {
    fn default() -> Self { Self::NONE }
}

impl Default for RawBodyContentType {
    fn default() -> Self { Self::Text(String::default()) }
}

impl Default for BodyCursorPosition {
    fn default() -> Self { Self::NONE }
}

impl Body {
    pub fn body_content(&self) -> &BodyContent { &self.body_content }
    pub fn set_body_content(&mut self, body_content: BodyContent) {
        self.body_content = body_content;
    }

    pub fn cursor_position(&self) -> &BodyCursorPosition { &self.cursor_position }
    pub fn set_cursor_position(&mut self, bcp: BodyCursorPosition) {
        self.cursor_position = bcp;
    }
}

