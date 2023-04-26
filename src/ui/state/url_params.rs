#[derive(Clone)]
pub struct UrlParams {
    params: Vec<Param>,

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

#[derive(Clone, Default)]
pub struct Param {
    name: String,
    value: String,
}

impl Default for UrlParams {
    fn default() -> Self { Self {
        params: vec![Param::default()],
        active_param_row: 0,
        active_param_col: 0,
    } }
}

impl UrlParams {
    pub fn params(&self) -> &Vec<Param> { &self.params }
    pub fn get_param(&self, i: u16) -> &Param { &self.params[i as usize] }
    pub fn get_param_name(&self, i: u16) -> String {
        self.get_param(i).clone().name()
    }
    pub fn get_param_value(&self, i: u16) -> String {
        self.get_param(i).clone().value()
    }
    /// Adds a new param to the end of `params`.
    /// Param limit is set to `1000`.
    pub fn add_param(&mut self, param: Param) {
        if self.params.len() == 1000 {
            return;
        }

        self.params.push(param);
    }
    /// Inserts a new param to the `pos` position in `params`.
    /// Param limit is set to `1000`.
    pub fn insert_param(&mut self, pos: u16, param: Param) {
        if self.params.len() == 1000 || pos == 1000{
            return;
        }

        self.params.insert(pos as usize, param);
    }
    /// Removes param in `pos` position.
    pub fn remove_param(&mut self, pos: u16) {
        self.params.remove(pos as usize);

        if self.params.len() == 0 {
            self.params.push(Param::default());
        }
    }

    pub fn active_param_row(&self) -> u16 { self.active_param_row }
    pub fn set_active_param_row(&mut self, row: u16) {
        self.active_param_row = row;
    }
    pub fn active_param_col(&self) -> u8 { self.active_param_col }
    pub fn set_active_param_col(&mut self, col: u8) {
        self.active_param_col = col;
    }

    //TODO: implement append, pop and clear functions for param name/value
    pub fn param_name_update(&mut self, param_index: u16, name: String) {
        self.params[param_index as usize].set_name(name);
    }
    pub fn param_value_update(&mut self, param_index: u16, value: String) {
        self.params[param_index as usize].set_value(value);
    }
}

impl Param {
    pub fn name(self) -> String { self.name.clone() }
    pub fn set_name(&mut self, name: String) { self.name = name }
    pub fn value(self) -> String { self.value.clone() }
    pub fn set_value(&mut self, value: String) { self.value = value }
}

