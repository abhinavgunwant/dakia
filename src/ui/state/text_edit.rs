//! The state associated to the multi-line TextInput widget.

use log::info;

pub enum TextEditMoveDirection {
    Up, Right, Down, Left, End, Home,
}

#[derive(Clone)]
pub struct TextEditState {
    /// Text lines. Each element in the vector is a line.
    text: Vec<String>,

    /// Cursor position in current line
    cursor_pos: u16,

    /// The current line number that is being edited
    line_number: u16,

    sel_start_pos: u16,
    sel_end_pos: u16,
    scroll_offset: u16,
}

impl Default for TextEditState {
    fn default() -> TextEditState {
        TextEditState {
            text: vec![String::default()],
            cursor_pos: 0,
            line_number: 0,
            sel_start_pos: 0,
            sel_end_pos: 0,
            scroll_offset: 0,
        }
    }
}

impl TextEditState {
    pub fn text(&self) -> String { self.text.join("\n") }
    pub fn text_vec(&self) -> Vec<String> { self.text.clone() }
    pub fn set_text(&mut self, text: Vec<String>) { self.text = text; }

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

    fn current_line_len(&self) -> usize {
        match self.text.get(self.line_number as usize) {
            Some (line) => line.len(),
            None => 0,
        }
    }

    /// Deletes character to the left of the cursor.
    /// If there are no characters to the left of cursor, delete the line if
    /// there are lines.
    pub fn delete_char(&mut self) {
        if self.text.len() > 0 {
            if self.cursor_pos() > 0 {
                let pos = self.cursor_pos as usize;
                match self.text.get_mut(self.line_number as usize) {
                    Some(s) => {
                        self.cursor_pos -= 1;
                        s.replace_range(pos-1..pos, "");
                    }
                    None => {}
                }
            } else {
                let s = self.text[self.line_number as usize].clone();
                self.text.remove(self.line_number as usize);
                self.line_number -= 1;
                self.cursor_pos = self.current_line_len() as u16;

                match self.text.get_mut(self.line_number as usize) {
                    Some (line_mut) => { line_mut.push_str(s.as_str()); }
                    None => {}
                }
            }
        }
    }

    pub fn delete_char_to_right(&mut self) {
        if self.text.len() > 0 {
            let curr_line_length = self.text[
                    self.line_number as usize
                ].len() as u16;

            if self.cursor_pos < curr_line_length {
                let pos = self.cursor_pos as usize;
                match self.text.get_mut(self.line_number as usize) {
                    Some (s) => {
                        s.replace_range(pos..pos+1, "");
                    }
                    None => {}
                }
            } else {
                self.text.remove((self.line_number+1) as usize);
            }
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        match self.text.get_mut(self.line_number as usize) {
            Some (s) => {
                s.insert(self.cursor_pos as usize, ch);
            }
            None => {}
        }

        self.cursor_pos += 1;
    }

    pub fn scroll_offset(&self) -> u16 { self.scroll_offset.clone() }
    pub fn set_scroll_offset(&mut self, scroll_offset: u16) {
        self.scroll_offset = scroll_offset;
    }

    pub fn line_number(&self) -> u16 { self.line_number.clone() }
    pub fn set_line_number(&mut self, line_number: u16) {
        self.line_number = line_number;
    }

    pub fn move_cursor(&mut self, move_direction: TextEditMoveDirection, word: bool) {
        match move_direction {
            TextEditMoveDirection::Up => {
                if self.line_number > 0 {
                    self.line_number -= 1;

                    let line_len = self.current_line_len() as u16;

                    if self.cursor_pos > line_len {
                        self.cursor_pos = line_len;
                    }
                }
            }
            TextEditMoveDirection::Down => {
                if self.line_number < self.text.len() as u16 {
                    self.line_number += 1;

                    let line_len = self.current_line_len() as u16;

                    if self.cursor_pos > line_len {
                        self.cursor_pos = line_len;
                    }
                }
            }
            TextEditMoveDirection::Right => {
                let end: u16 = self.current_line_len() as u16;

                if self.cursor_pos < end {
                    if word {
                        let s = &self.text[self.line_number as usize];

                        self.cursor_pos += 1;

                        while self.cursor_pos < end {
                            let c = s.chars().nth(self.cursor_pos as usize)
                                .unwrap();

                            if c == ' ' {
                                break;
                            }

                            self.cursor_pos += 1;
                        }
                    } else {
                        self.cursor_pos += 1;
                    }
                }
            }
            TextEditMoveDirection::Left => {
                if self.cursor_pos > 0 {
                    if word {
                        let s = &self.text[self.line_number as usize];
                        while self.cursor_pos > 0 {
                            self.cursor_pos -= 1;

                            let c = s.chars().nth(self.cursor_pos as usize)
                                .unwrap();

                            if c == ' ' {
                                break;
                            }
                        }
                    } else {
                        self.cursor_pos -= 1;
                    }
                }
            }
            TextEditMoveDirection::End => {
                self.cursor_pos = self.current_line_len() as u16;
            }
            TextEditMoveDirection::Home => {
                self.cursor_pos = 0;
            }
        }
    }

    /// Adds new line
    ///
    /// When the cursor is in-between a line, then the string to the right of
    /// the cursor. When cursor is at the end of line, a new empty line is
    /// added at the end of text.
    pub fn new_line(&mut self) {
        let s = &self.text[self.line_number as usize];

        if s.len() > self.cursor_pos.clone() as usize {
            let s_clone = s.clone();
            let l_num = self.line_number as usize;
            let (left, right) = s_clone.split_at(self.cursor_pos as usize);

            self.text.remove(l_num);
            self.text.insert(l_num, String::from(left));
            self.text.insert(l_num + 1, String::from(right));
        } else {
            self.text.push(String::default());
        }

        self.line_number += 1;
        self.cursor_pos = 0;
    }
}

