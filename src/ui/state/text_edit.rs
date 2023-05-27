//! The state associated to the multi-line TextInput widget.

pub enum TextEditMoveDirection {
    Up, Right, Down, Left, End, Home,
}

#[derive(Clone, Default)]
pub struct TextEditState {
    text: String,
    cursor_pos: u16,
    sel_start_pos: u16,
    sel_end_pos: u16,
    scroll_offset: u16,
}

impl TextEditState {
    pub fn text(&self) -> String { self.text.clone() }
    pub fn set_text(&mut self, text: String) { self.text = text; }

    pub fn cursor_pos(&self) -> u16 { self.cursor_pos.clone() }
    pub fn set_cursor_pos(&mut self, cursor_pos: u16) {
        self.cursor_pos = cursor_pos;
    }

    pub fn sel_start_pos(&self) -> u16 { self.sel_start_pos.clone() }
    pub fn set_sel_start_pos(&mut self, sel_start_pos: u16) {
        self.sel_start_pos = sel_start_pos;
    }

    pub fn sel_end_pos(&self) -> u16 { self.sel_end_pos.clone() }
    pub fn set_sel_end_pos(&mut self, sel_end_pos: u16) {
        self.sel_end_pos = sel_end_pos;
    }

    pub fn delete_char(&mut self) {
        if self.text.len() > 0 {
            let pos = self.cursor_pos as usize;

            self.text.replace_range(pos-1..pos, "");
            self.cursor_pos -= 1;
        }
    }

    pub fn delete_char_to_right(&mut self) {
        if self.cursor_pos < self.text.len() as u16 {
            let pos = self.cursor_pos as usize;

            self.text.replace_range(pos..pos+1, "");
            //self.cursor_pos -= 1;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.text.insert(self.cursor_pos as usize, ch);
        self.cursor_pos += 1;
    }

    pub fn scroll_offset(&self) -> u16 { self.scroll_offset.clone() }
    pub fn set_scroll_offset(&mut self, scroll_offset: u16) {
        self.scroll_offset = scroll_offset;
    }

    pub fn move_cursor(&mut self, move_direction: TextEditMoveDirection, word: bool) {
        match move_direction {
            TextEditMoveDirection::Up => {}
            TextEditMoveDirection::Down => {}
            TextEditMoveDirection::Right => {
                if self.cursor_pos < self.text.len() as u16 {
                    // TODO: handle word here
                    self.cursor_pos += 1;
                }
            }
            TextEditMoveDirection::Left => {
                if self.cursor_pos > 0 {
                    // TODO: handle word here
                    self.cursor_pos -= 1;
                }
            }
            TextEditMoveDirection::End => {
                self.cursor_pos = self.text.len() as u16;
            }
            TextEditMoveDirection::Home => {
                self.cursor_pos = 0;
            }
        }
    }
}

