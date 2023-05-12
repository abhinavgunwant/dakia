use std::slice::Iter;
use super::{kv_data::KVData, kv_tab_state::KVTabState};

const EMPTY_STRING: String = String::new();
const DEFAULT_KV_DATA: KVData = KVData::default_const();
const DEFAULT_BODY_CONTENT_TYPE: RawBodyContentType = RawBodyContentType::Text(EMPTY_STRING);

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

    pub fn body_content_types(&self) -> Vec<String> {
        let mut v: Vec<String> = vec![];

        v.push(String::from("None"));
        v.push(String::from("Form Data"));
        v.push(String::from("Form URL-encoded"));
        v.push(String::from("Raw"));

        v
    }

    pub fn body_raw_content_types(&self) -> Vec<String> {
        let mut v: Vec<String> = vec![];

        v.push(String::from("Text"));
        v.push(String::from("JSON"));
        v.push(String::from("HTML"));
        v.push(String::from("XML"));

        v
    }
}

impl BodyContent {
    pub fn to_string(&self) -> String {
        match self {
            BodyContent::NONE => String::from("None"),
            BodyContent::FormData(_a) => String::from("Form Data"),
            BodyContent::FormURLEncoded(_a) => String::from("Form URL-encoded"),
            BodyContent::Raw(_a) => String::from("Raw"),
        }
    }

    pub fn from_string(input: String) -> Self {
        BodyContent::from_str(&input)
    }

    pub fn from_str(input: &str) -> Self {
        match input {
            "None" => BodyContent::NONE,
            "Form Data" => BodyContent::FormData(KVData::default()),
            "Form URL-encoded" => BodyContent::FormURLEncoded(KVData::default()),
            "Raw" => BodyContent::Raw(RawBodyContentType::default()),
            _ => BodyContent::default(),
        }
    }

    pub fn iter() -> Iter<'static, BodyContent> {
        static BODY_CONTENT_TYPES: [BodyContent; 4] = [
            BodyContent::NONE,
            BodyContent::FormData(DEFAULT_KV_DATA),
            BodyContent::FormURLEncoded(DEFAULT_KV_DATA),
            BodyContent::Raw(DEFAULT_BODY_CONTENT_TYPE),
        ];

        BODY_CONTENT_TYPES.iter()
    }
}

