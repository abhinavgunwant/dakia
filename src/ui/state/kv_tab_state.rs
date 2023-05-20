//! State of the tab that shows and sets key-value data.
//! Used in "URL Params" and "Headers" tabs.
//! Keeps concerns separate from `QueryParams` and `HeaderMap`.
#[derive(Clone, PartialEq)]
pub struct KVTabState {
    /// The param being edited/active (index of `params`)
    active_row: u16,

    /// The current active param "column". This is purely for ui use
    /// Here are the column values and their meanings:
    /// * 0 - the "name" of the param is highlighted or being edited
    /// * 1 - the "value" of the param is highlighted or being edited
    /// * 2 - the add new param button is highlighted
    /// * 3 - the remove param button is highlighted
    active_col: u8,

    scroll_pos: u16,
}

impl Default for KVTabState {
    fn default() -> Self { Self {
        active_row: 0,
        active_col: 0,
        scroll_pos: 0,
    } }
}

impl KVTabState {
    pub fn active_row(&self) -> u16 { self.active_row }
    pub fn set_active_row(&mut self, row: u16) {
        self.active_row = row;
    }

    pub fn active_col(&self) -> u8 { self.active_col }
    pub fn set_active_col(&mut self, col: u8) {
        self.active_col = col;
    }

    pub fn scroll_pos(&self) -> u16 { self.scroll_pos }
    pub fn set_scroll_pos(&mut self, pos: u16) {
        self.scroll_pos = pos;
    }
}

