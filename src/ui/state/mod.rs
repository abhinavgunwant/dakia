//! State-related things

pub mod request_tabs;
pub mod kv_data;
pub mod kv_tab_state;
pub mod url;
pub mod response;
pub mod app_status;
pub mod body;

//use std::fmt::{ Display, Formatter, Result as FResult };
use reqwest::Method;
use request_tabs::RequestTabs;
use kv_tab_state::KVTabState;
use kv_data::KVData;
use url::Url;
use body::Body;

use self::{response::Response, app_status::AppStatus};

const METHOD_ALLOWED_CHARS: &str = "GPUDHOCAT";

/// Represents app state.
#[derive(Clone)]
pub struct UiState {
    /// The URL that user types in the URL bar.
    url: String,
    
    /// The deconstructed URL
    url_deconst: Url,

    /// The state of the "Body" tab in request section.
    body: Body,

    /// The current [EditorMode].
    editor_mode: EditorMode,

    /// The current HTTP request [Method].
    method: Method,

    /// The current request tab that is active.
    active_request_tab: RequestTabs,

    /// Counts the current request.
    /// Incremented with each request. Used for re-caching the response.
    request_counter: u8,
    request_headers: Vec<KVData>,
    request_headers_ui: KVTabState,

    /// The response output of the request.
    /// **Note:** Value is `None` until the first request is made.
    response: Response,

    /// Status code of the request.
    /// **Note:** Value is `None` until the first request is made.
    response_status_code: Option<u16>,

    /// Currently active [UIElement].
    active_element: UIElement,

    /// Current [InputMode].
    input_mode: InputMode,

    /// HTTP url query parameters
    query_params_ui: KVTabState,

    app_status: AppStatus,
    app_error: Option<String>,
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

impl Default for UiState {
    fn default() -> Self {
        Self {
            url: String::default(),
            url_deconst: Url::default(),
            editor_mode: EditorMode::default(),
            method: Method::default(),
            active_request_tab: RequestTabs::default(),
            request_counter: 0,
            request_headers: vec![KVData::default()],
            request_headers_ui: KVTabState::default(),
            response: Response::default(),
            response_status_code: None,
            active_element: UIElement::default(),
            input_mode: InputMode::default(),
            query_params_ui: KVTabState::default(),
            app_status: AppStatus::default(),
            app_error: None,
        }
    }
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

    pub fn url_deconst(&self) -> &Url { &self.url_deconst }
    pub fn url_deconst_mut(&mut self) -> &mut Url { &mut self.url_deconst }

    /// Gets the current [Method].
    pub fn method(&self) -> Method { self.method.clone() }
    /// Sets the current [Method].
    pub fn set_method(&mut self, method: Method) { self.method = method; }
    pub fn set_method_from_val(&mut self, val: u8) {
        match val {
            0 => self.set_method(Method::GET),
            1 => self.set_method(Method::POST),
            2 => self.set_method(Method::PUT),
            3 => self.set_method(Method::DELETE),
            4 => self.set_method(Method::HEAD),
            5 => self.set_method(Method::OPTIONS),
            6 => self.set_method(Method::CONNECT),
            7 => self.set_method(Method::PATCH),
            8 => self.set_method(Method::TRACE),
            _ => self.set_method(Method::GET),
        }
    }
    pub fn set_method_from_char(&mut self, c: char) {
        let c_ = c.to_ascii_uppercase();
        if METHOD_ALLOWED_CHARS.contains(c_.to_string().as_str()) {
            match c_ {
                'G' => { self.set_method(Method::GET); },
                'P' => { self.set_method(Method::POST); },
                'U' => { self.set_method(Method::PUT); },
                'D' => { self.set_method(Method::DELETE); },
                'H' => { self.set_method(Method::HEAD); },
                'O' => { self.set_method(Method::OPTIONS); },
                'C' => { self.set_method(Method::CONNECT); },
                'A' => { self.set_method(Method::PATCH); },
                'T' => { self.set_method(Method::TRACE); },
                _ => { self.set_method(Method::GET); },
            }

            return;
        }

        self.set_method(Method::GET);
    }

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

    pub fn response(&self) -> &Response { &self.response }
    pub fn response_mut(&mut self) -> &mut Response { &mut self.response }

    pub fn request_counter(&self) -> u8 { self.request_counter }
    pub fn increment_request_counter(&mut self) { self.request_counter += 1 }

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

    pub fn request_headers(&self) -> &Vec<KVData> { &self.request_headers }
    pub fn request_headers_mut(&mut self) -> &mut Vec<KVData> {
        &mut self.request_headers
    }
    
    pub fn request_headers_ui(&self) -> KVTabState {
        self.request_headers_ui.clone()
    }
    pub fn request_headers_ui_mut(&mut self) -> &mut KVTabState {
        &mut self.request_headers_ui
    }
    
    pub fn query_params_ui(&self) -> KVTabState {
        self.query_params_ui.clone()
    }
    pub fn query_params_ui_mut(&mut self) -> &mut KVTabState {
        &mut self.query_params_ui
    }

    pub fn app_status(&self) -> &AppStatus { &self.app_status }
    pub fn set_app_status(&mut self, app_status: AppStatus) {
        self.app_status = app_status;
    }

    pub fn app_error(&self) -> &Option<String> { &self.app_error }
    pub fn set_app_error(&mut self, error_str: String) {
        self.app_error = Some(error_str);
    }

    pub fn insert_url_param(&mut self, pos: u16, param: KVData) {
        self.url_deconst.insert_param(pos, param);
    }
    pub fn remove_url_param(&mut self, pos: u16) {
        self.url_deconst.remove_param(pos);
    }

    pub fn insert_header(&mut self, pos: u16, param: KVData) {
        self.request_headers.insert(pos as usize, param);
    }
    pub fn remove_header(&mut self, pos: u16) {
        self.request_headers.remove(pos as usize);

        if self.request_headers.len() == 0 {
            self.request_headers.push(KVData::default());
        }
    }
}

