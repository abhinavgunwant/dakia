//! State of the UI in Query params tab.
//! Keeps concerns separate from `QueryParams`.
#[derive(Clone)]
pub struct QueryParamsUi {
    /// The param being edited/active (index of `params`)
    active_param_row: u16,

    /// The current active param "column". This is purely for ui use
    /// Here are the column values and their meanings:
    /// * 0 - the "name" of the param is highlighted or being edited
    /// * 1 - the "value" of the param is highlighted or being edited
    /// * 2 - the add new param button is highlighted
    /// * 3 - the remove param button is highlighted
    active_param_col: u8,
}

impl Default for QueryParamsUi {
    fn default() -> Self { Self {
        active_param_row: 0,
        active_param_col: 0,
    } }
}

impl QueryParamsUi {
    pub fn active_param_row(&self) -> u16 { self.active_param_row }
    pub fn set_active_param_row(&mut self, row: u16) {
        self.active_param_row = row;
    }
    pub fn active_param_col(&self) -> u8 { self.active_param_col }
    pub fn set_active_param_col(&mut self, col: u8) {
        self.active_param_col = col;
    }
}

