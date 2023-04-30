#[derive(Clone, Default)]
pub struct QueryParam {
    name: String,
    value: String,
}

impl QueryParam {
    pub fn new(name: String, value: String) -> Self {
        QueryParam { name, value }
    }

    pub fn from_string(input: String) -> Self {
        Self::from_str(&input)
    }

    pub fn from_str(input: &str) -> Self {
        if input.is_empty() {
            return QueryParam::default();
        }

        let inp_vec: Vec<&str> = input.split('=').collect();

        let name = String::from(inp_vec[0]);
        let mut value = String::default();

        if inp_vec.len() == 2 {
            value.push_str(inp_vec[1]);
        }

        QueryParam { name, value }
    }

    pub fn to_string(&self) -> String {
        if !self.name.is_empty() && !self.value.is_empty() {
            return format!("{}={}", self.name, self.value);
        }

        if !self.name.is_empty() {
            return self.name();
        }

        String::from("")
    }

    pub fn name(&self) -> String { self.name.clone() }
    pub fn set_name(&mut self, name: String) { self.name = name; }

    pub fn value(&self) -> String { self.value.clone() }
    pub fn set_value(&mut self, value: String) { self.value = value; }
}

