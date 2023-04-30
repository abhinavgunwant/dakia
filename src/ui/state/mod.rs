//! State-related things

pub mod request_tabs;
pub mod query_param;
pub mod query_params_ui;
pub mod url;

use std::fmt::{ Display, Formatter, Result as FResult };
use request_tabs::RequestTabs;
use query_params_ui::QueryParamsUi;
use url::Url;

/// Represents app state.
#[derive(Clone, Default)]
pub struct UiState {
    /// The URL that user types in the URL bar.
    url: String,
    
    /// The deconstructed URL
    url_deconst: Url,

    /// The current [EditorMode].
    editor_mode: EditorMode,

    /// The current HTTP request [Method].
    method: Method,

    /// The current request tab that is active.
    active_request_tab: RequestTabs,

    /// The response output of the request.
    /// **Note:** Value is `None` until the first request is made.
    response: Option<String>,

    /// Status code of the request.
    /// **Note:** Value is `None` until the first request is made.
    response_status_code: Option<u16>,

    /// Currently active [UIElement].
    active_element: UIElement,

    /// Current [InputMode].
    input_mode: InputMode,

    /// HTTP url query parameters
    query_params_ui: QueryParamsUi,
}

/// An enum representing all the ui elements that can be seen on the screen.
#[derive(Clone, Copy, PartialEq)]
pub enum UIElement {
    Method = 0,
    URL = 1,
    SendButton = 2,
    RequestTabsHead = 3,
    RequestTabsElem = 4,
    ResponseArea = 5,
}

impl Default for UIElement {
    fn default() -> Self { UIElement::URL }
}

impl UIElement {
    /// Returns a `UIElement` variant against the value supplied.
    fn from_val(val: u8) -> Self {
        match val {
            0 => UIElement::Method,
            1 => UIElement::URL,
            2 => UIElement::SendButton,
            3 => UIElement::RequestTabsHead,
            4 => UIElement::RequestTabsElem,
            5 => UIElement::ResponseArea,
            _ => UIElement::ResponseArea,
        }
    }
}

/// List of HTTP methods
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
        write!(f, "Method::{}", self.clone().get_str_label())
    }
}

impl Method {
    /// Returns the string label for the method variants. Used to render the
    /// different methods in the ui.
    pub fn get_str_label(self) -> String {
        match self {
            Method::GET => String::from("GET"),
            Method::POST => String::from("POST"),
            Method::PUT => String::from("PUT"),
            Method::DELETE => String::from("DELETE"),
            Method::HEADER => String::from("HEADER"),
        }
    }

    /// Returns `Method` variant against the value supplied.
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

/// Represents the editor mode.
/// There can be 2 editor modes:
/// * **Normal** - This is the default editor mode. The user uses the tab and
/// arrow keys on their keyboard to move around. Once a field is selected, it
/// can be edited directly.
///
/// * **\*VIM** - This is a future editor mode with vim movements and modes like
/// `NORMAL`, `INSERT` etc.
///
/// **Notes**
/// * **\*** - VIM editor mode is yet to be implemented! It doesn't work as
/// of now.
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
    /// Gets the URL
    pub fn url(&self) -> String { self.url.clone() }
    /// Sets the URL
    pub fn set_url(&mut self, url: String) { self.url = url; }
    /// Appends `chr` at the end of the URL.
    pub fn append_url(&mut self, chr: char) { self.url.push(chr); }
    pub fn append_url_string(&mut self, url_string: String) {
        self.url.push_str(url_string.as_str());
    }
    /// Pops the last character of the URL.
    pub fn pop_url(&mut self) { self.url.pop(); }
//    pub fn update_url_with_deconst(&mut self) {
//        let new_url = self.url_deconst().to_string();
//        self.set_url(new_url);
//    }

    pub fn url_deconst(&self) -> &Url { &self.url_deconst }
    pub fn url_deconst_mut(&mut self) -> &mut Url { &mut self.url_deconst }

    /// Gets the current [Method].
    pub fn method(&mut self) -> Method { self.method.clone() }
    /// Sets the current [Method].
    pub fn set_method(&mut self, method: Method) { self.method = method; }

    /// Gets the current [EditorMode].
    pub fn editor_mode(self) -> EditorMode { self.editor_mode }
    /// Sets the current [EditorMode].
    pub fn set_editor_mode(&mut self, editor_mode: EditorMode) {
        self.editor_mode = editor_mode;
    }

    /// Gets the current [InputMode].
    pub fn input_mode(&self) -> InputMode { self.input_mode }
    /// Sets the current [InputMode].
    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
    }

    /// Gets the active [UIElement].
    pub fn active_element(&self) -> &UIElement { &self.active_element }
    /// Sets the active [UIElement].
    pub fn set_active_element(&mut self, act_elem: UIElement) {
        self.active_element = act_elem;
    }
    /// Changes the `active_element` to the next [UIElement] variant.
    /// When the current `active_element` is the last [UIElement] variant (or
    /// has the largest integer value), this keeps the same element active.
    pub fn activate_next_element(&mut self) {
        let n = *self.active_element() as u8;
        self.set_active_element(UIElement::from_val(n + 1));
    }
    /// Changes the `active_element` to the previous [UIElement] variant.
    /// When the current `active_element` is the first [UIElement] variant (or
    /// has the smallest integer value), this keeps the same element active.
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

    pub fn active_request_tab(&self) -> &RequestTabs {
        &self.active_request_tab
    }
    pub fn set_active_request_tab(&mut self, rt: RequestTabs) {
        self.active_request_tab = rt;
    }

    pub fn activate_next_req_tab(&mut self) {
        let n = *self.active_request_tab() as u8;
        self.set_active_request_tab(RequestTabs::from_val(n + 1));
    }
    pub fn activate_previous_req_tab(&mut self) {
        let n = *self.active_request_tab() as u8;

        if n == 0 {
            return;
        }

        self.set_active_request_tab(RequestTabs::from_val(n - 1));
    }
    
    pub fn query_params_ui(&self) -> QueryParamsUi { self.query_params_ui.clone() }
    pub fn query_params_ui_mut(&mut self) -> &mut QueryParamsUi {
        &mut self.query_params_ui
    }
}

