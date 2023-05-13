use std::slice::Iter;
use super::{ kv_data::KVData, kv_tab_state::KVTabState };

const EMPTY_STRING: String = String::new();
const DEFAULT_KV_DATA: KVData = KVData::default_const();
const DEFAULT_BODY_CONTENT_TYPE: RawBodyContentType = RawBodyContentType::Text(EMPTY_STRING);

/// The UI elements in the body tab (e.g. The "Content Type" selection).
/// **Note:** the `bool` value in the variant defines whether the body ui
/// element is active or not.
#[derive(Clone, PartialEq)]
pub enum BodyUIElement {
    /// The "Content Type" select widget. `true` if it's active.
    /// params
    /// - bool - whether the Select widget is open
    ContentType(bool),

    /// The "Raw Content Type" select widget. `true` if it's active.
    /// params
    /// - bool - whether the Select widget is open
    RawContentType(bool),

    TextArea,
}

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

#[derive(Clone)]
pub struct Body {
    active_body_element: BodyUIElement,
    body_content: BodyContent,

    /// The index of the element selected in the Body Content select.
    body_content_sel_index: u8,
    body_content_options: Vec<String>,
    raw_body_content: RawBodyContentType,

    /// The index of the element selected in the Raw Body Content select.
    raw_body_content_sel_index: u8,
    cursor_position: BodyCursorPosition,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            active_body_element: BodyUIElement::default(),
            body_content: BodyContent::default(),
            body_content_sel_index: 0,
            body_content_options: BodyContent::as_string_vec(),
            raw_body_content: RawBodyContentType::default(),
            raw_body_content_sel_index: 0,
            cursor_position: BodyCursorPosition::default(),
        }
    }
}

impl Default for BodyUIElement {
    fn default() -> Self { Self::ContentType(false) }
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
    pub fn active_body_element(&self) -> &BodyUIElement {
        &self.active_body_element
    }
    pub fn set_active_body_element(&mut self, active_body_el: BodyUIElement) {
        self.active_body_element = active_body_el;
    }

    pub fn body_content(&self) -> &BodyContent { &self.body_content }
    pub fn set_body_content(&mut self, body_content: BodyContent) {
        self.body_content = body_content;
    }

    pub fn body_content_sel_index(&self) -> &u8 { &self.body_content_sel_index }
    pub fn set_body_content_sel_index(&mut self, body_content_sel_index: u8) {
        self.body_content_sel_index = body_content_sel_index;
    }

    pub fn body_content_options(&self) -> Vec<String> { self.body_content_options.clone() }
    pub fn set_body_content_options(&mut self, body_content_options: Vec<String>) {
        self.body_content_options = body_content_options;
    }

//    pub fn body_content_ui_state(&self) -> &SelectData { &self.body_content_ui_state }
//    pub fn set_body_content_ui_state(&mut self, body_content_ui_state: SelectData) {
//        self.body_content_ui_state = body_content_ui_state;
//    }

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

impl BodyUIElement {
    pub fn is_opened(&self) -> bool {
        match self {
            Self::ContentType(opened) => *opened,
            Self::RawContentType(opened) => *opened,
            _ => false,
        }
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

    pub fn as_string_vec() -> Vec<String> {
        let mut v: Vec<String> = vec![];

        for item in Self::iter() {
            v.push(item.to_string());
        }

        v
    }
}

