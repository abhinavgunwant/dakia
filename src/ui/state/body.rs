use std::slice::Iter;
use super::{ kv_data::KVData, kv_tab_state::KVTabState };

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

#[derive(Clone, PartialEq)]
pub enum BodyContent {
    NONE,
    FormData,
    FormURLEncoded,
    Text,
    Json,
    Html,
    Xml,
//    Raw(RawBodyContentType),
}

/// To help with syntax highlighting later-on!
#[derive(Clone, PartialEq)]
pub enum RawBodyContentType {
    Text,
    Json,
    Html,
    Xml,
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

    /// The body "Content Type"
    body_content: BodyContent,

    /// The index of the element selected in the Body Content select.
    body_content_sel_index: u8,
    body_content_options: Vec<String>,
    body_content_scroll_offset: u8,
    raw_body_content: RawBodyContentType,
    raw_body_content_options: Vec<String>,

    /// The index of the element selected in the Raw Body Content select.
    raw_body_content_sel_index: u8,
    cursor_position: BodyCursorPosition,

    /// The KV data for the body.
    kv_data: Vec<KVData>,

    /// Text data for the body
    text_data: String,

    /// The length of the select widget popup content length.
    disp_content_len: u8,
}

impl Default for Body {
    fn default() -> Self {
        let kv_data: Vec<KVData> = vec![];

        Self {
            active_body_element: BodyUIElement::default(),
            body_content: BodyContent::default(),
            body_content_sel_index: 0,
            body_content_options: BodyContent::as_string_vec(),
            body_content_scroll_offset: 0,
            raw_body_content: RawBodyContentType::default(),
            raw_body_content_options: RawBodyContentType::as_string_vec(),
            raw_body_content_sel_index: 0,
            cursor_position: BodyCursorPosition::default(),
            kv_data,
            disp_content_len: 5,
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
    fn default() -> Self { Self::Text }
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

    pub fn raw_body_content(&self) -> &RawBodyContentType { &self.raw_body_content }
    pub fn set_raw_body_content(&mut self, raw_body_content: RawBodyContentType) {
        self.raw_body_content = raw_body_content;
    }

    pub fn raw_body_content_options(&self) -> Vec<String> { self.raw_body_content_options.clone() }
    pub fn set_raw_body_content_options(&mut self, raw_body_content_options: Vec<String>) {
        self.raw_body_content_options = raw_body_content_options;
    }

    pub fn raw_body_content_sel_index(&self) -> &u8 { &self.raw_body_content_sel_index }
    pub fn set_raw_body_content_sel_index(&mut self, raw_body_content_sel_index: u8) {
        self.raw_body_content_sel_index = raw_body_content_sel_index;
    }

    pub fn cursor_position(&self) -> &BodyCursorPosition { &self.cursor_position }
    pub fn set_cursor_position(&mut self, bcp: BodyCursorPosition) {
        self.cursor_position = bcp;
    }

    pub fn disp_content_len(&self) -> &u8 { &self.disp_content_len }
    pub fn set_disp_content_len(&mut self, disp_content_len: u8) {
        self.disp_content_len = disp_content_len;
    }

    pub fn body_content_scroll_offset(&self) -> &u8 {
        &self.body_content_scroll_offset
    }
    pub fn set_body_content_scroll_offset(&mut self, offset: u8) {
        self.body_content_scroll_offset = offset;
    }
}

//impl BodyUIElement {
//    pub fn is_opened(&self) -> bool {
//        match self {
//            Self::ContentType(opened) => *opened,
//            Self::RawContentType(opened) => *opened,
//            _ => false,
//        }
//    }
//}

impl BodyContent {
    pub fn to_string(&self) -> String {
        match self {
            BodyContent::NONE => String::from("None"),
            BodyContent::FormData => String::from("Form Data"),
            BodyContent::FormURLEncoded => String::from("Form URL-encoded"),
            BodyContent::Text => String::from("Text"),
            BodyContent::Json => String::from("Json"),
            BodyContent::Html => String::from("Html"),
            BodyContent::Xml => String::from("Xml"),
            // BodyContent::Raw(_a) => String::from("Raw"),
        }
    }

    pub fn from_string(input: String) -> Self {
        BodyContent::from_str(&input)
    }

    pub fn from_str(input: &str) -> Self {
        match input {
            "None" => BodyContent::NONE,
            "Form Data" => BodyContent::FormData,
            "Form URL-encoded" => BodyContent::FormURLEncoded,
            "Text" => BodyContent::Text,
            "Json" => BodyContent::Json,
            "Html" => BodyContent::Html,
            "Xml" => BodyContent::Xml,
            //"Raw" => BodyContent::Raw(RawBodyContentType::default()),
            _ => BodyContent::default(),
        }
    }

    pub fn iter() -> Iter<'static, BodyContent> {
        static BODY_CONTENT_TYPES: [BodyContent; 7] = [
            BodyContent::NONE,
            BodyContent::FormData,
            BodyContent::FormURLEncoded,
            BodyContent::Json,
            BodyContent::Text,
            BodyContent::Html,
            BodyContent::Xml,
            // BodyContent::Raw(RawBodyContentType::Text),
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

impl RawBodyContentType {
    pub fn to_string(&self) -> String {
        match self {
            RawBodyContentType::Text => String::from("Text"),
            RawBodyContentType::Json => String::from("JSON"),
            RawBodyContentType::Html => String::from("HTML"),
            RawBodyContentType::Xml => String::from("XML"),
        }
    }

    pub fn from_string(input: String) -> Self {
        RawBodyContentType::from_str(&input)
    }

    pub fn from_str(input: &str) -> Self {
        match input {
            "Text" => RawBodyContentType::Text,
            "JSON" => RawBodyContentType::Json,
            "HTML" => RawBodyContentType::Html,
            "XML" => RawBodyContentType::Xml,
            _ => RawBodyContentType::Text,
        }
    }

    pub fn iter() -> Iter<'static, RawBodyContentType> {
        static RAW_BODY_CONTENT_TYPES: [RawBodyContentType; 4] = [
            RawBodyContentType::Text,
            RawBodyContentType::Json,
            RawBodyContentType::Html,
            RawBodyContentType::Xml,
        ];

        RAW_BODY_CONTENT_TYPES.iter()
    }

    pub fn as_string_vec() -> Vec<String> {
        let mut v: Vec<String> = vec![];

        for item in Self::iter() {
            v.push(item.to_string());
        }

        v
    }
}

