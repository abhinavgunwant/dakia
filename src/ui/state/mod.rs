use std::fmt::{ Display, Formatter, Result as FResult };

#[derive(Clone, Default)]
pub struct UiState {
    url: String,
    editor_mode: EditorMode,
    method: Method,
    response: Option<String>,
    response_status_code: Option<u16>,
    active_element: UIElement, // the ui element that's currently active.
    input_mode: InputMode, // works only with editor_mode = VIM
}

/**
 * An enum representing the ui elements.
 * e.g. URL, Method, etc
 */
#[derive(Clone, Copy, PartialEq)]
pub enum UIElement {
    Method = 0, URL = 1,
}

impl Default for UIElement {
    fn default() -> Self { UIElement::URL }
}

impl UIElement {
    fn from_val(val: u8) -> Self {
        match val {
            x if x <= UIElement::Method as u8  => UIElement::Method,
            x if x >= UIElement::URL as u8 => UIElement::URL,
            _ => UIElement::URL,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Method {
    GET = 0,
    POST = 1,
    PUT = 2,
    DELETE = 3,
    HEADER = 4,
}

impl Default for Method {
    fn default() -> Self { Method::GET }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "Method::{}", self.clone().get_val())
    }
}

impl Method {
    pub fn get_val(self) -> String {
        match self {
            Method::GET => String::from("GET"),
            Method::POST => String::from("POST"),
            Method::PUT => String::from("PUT"),
            Method::DELETE => String::from("DELETE"),
            Method::HEADER => String::from("HEADER"),
        }
    }

    pub fn from_val(val: u8) -> Self {
        match val {
            x if x <= Method::GET as u8  => Method::GET,
            x if x == Method::POST as u8  => Method::POST,
            x if x == Method::PUT as u8  => Method::PUT,
            x if x == Method::DELETE as u8  => Method::DELETE,
            x if x >= Method::HEADER as u8 => Method::HEADER,
            _ => Method::GET,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum EditorMode {
    Normal,
    VIM,
}

impl Default for EditorMode {
    fn default() -> Self { EditorMode::Normal }
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    INSERT,
    VISUAL,
}

impl Default for InputMode {
    fn default() -> Self { InputMode::Normal }
}

impl UiState {
    pub fn url(&mut self) -> String { self.url.clone() }
    pub fn set_url(&mut self, url: String) { self.url = url; }
    pub fn append_url(&mut self, chr: char) { self.url.push(chr); }
    pub fn pop_url(&mut self) { self.url.pop(); }

    pub fn method(&mut self) -> Method { self.method.clone() }
    pub fn set_method(&mut self, method: Method) { self.method = method; }

    pub fn editor_mode(self) -> EditorMode { self.editor_mode }
    pub fn set_editor_mode(&mut self, editor_mode: EditorMode) {
        self.editor_mode = editor_mode;
    }

    pub fn input_mode(&self) -> InputMode { self.input_mode }
    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
    }

    pub fn active_element(&self) -> &UIElement { &self.active_element }
    pub fn set_active_element(&mut self, act_elem: UIElement) {
        self.active_element = act_elem;
    }
    pub fn activate_next_element(&mut self) {
        let n = *self.active_element() as u8;
        self.set_active_element(UIElement::from_val(n + 1));
    }
    pub fn activate_previous_element(&mut self) {
        let n = *self.active_element() as u8;

        if n == 0 {
            return;
        }

        self.set_active_element(UIElement::from_val(n - 1));
    }

    pub fn response(&self) -> &Option<String> { &self.response }
    pub fn set_response(&mut self, resp: Option<String>) { self.response = resp; }

    pub fn response_status_code(&self) -> &Option<u16> {
        &self.response_status_code
    }
    pub fn set_response_status_code(&mut self, status: Option<u16>) {
        self.response_status_code = status;
    }
}

