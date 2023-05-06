#[derive(Clone, Default)]

/// Handles the content shown in the Response Area
pub struct Response {
    /// The state of the request counter when the request was cached.
    cache_req_counter: u8,
    response: Vec<String>,
    initialized: bool,
    scroll_pos: u16,
}

impl Response {
    pub fn cache_req_counter(&self) -> u8 { self.cache_req_counter }
    pub fn set_cache_req_counter(&mut self, cache_req_counter: u8) {
        self.cache_req_counter = cache_req_counter;
    }

    pub fn response(&self) -> &Vec<String> { &self.response }
    pub fn set_response(&mut self, response: Vec<String>) {
        self.response = response;
        self.initialized = true;
    }
    pub fn from_str(&mut self, resp_text: String) {
        self.response = resp_text.split('\n').map(|s| s.to_string()).collect();
        self.initialized = true;
    }

    pub fn initialized(&self) -> bool { self.initialized }

    pub fn scroll_pos(&self) -> u16 { self.scroll_pos }
    pub fn set_scroll_pos(&mut self, pos: u16) { self.scroll_pos = pos; }
}

