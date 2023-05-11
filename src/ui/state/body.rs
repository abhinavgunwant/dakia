use super::{kv_data::KVData, kv_tab_state::KVTabState};

pub enum BodyContent {
    NONE,
    FormData(KVData),
    FormURLEncoded(KVData),
    Raw(RawBodyContenType),
}

/// To help with syntax highlighting later-on!
pub enum RawBodyContenType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

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
    fn default(self) -> Self { Self::NONE }
}

impl default for RawBodyContentType {
    fn default(self) -> Self { Self::Text(String::default()) }
}

impl BodyCursorPosition {
    fn default(self) -> Self { Self::NONE }
}

impl Body {
    pub fn body_content(&self) -> BodyContentType { self.body_content }
    pub fn set_body_content(&mut self, body_content: BodyContent) {
        self.body_content = body_content;
    }

    pub fn cursor_position(&self) -> BodyCursorPosition { self.cursor_position }
    pub fn set_cursor_position(&mut self, bcp: BodyCursorPosition) {
        self.cursor_position = bcp;
    }
}

