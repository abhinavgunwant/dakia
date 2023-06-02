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

    /// (line, caracter) position for start of selection
    sel_start_pos: (u16, u16),

    /// (line, caracter) position for end of selection
    sel_end_pos: (u16, u16),
    scroll_offset: u16,
    selecting: bool,
}

impl Default for TextEditState {
    fn default() -> TextEditState {
        TextEditState {
            text: vec![String::default()],
            cursor_pos: 0,
            line_number: 0,
            sel_start_pos: (0, 0),
            sel_end_pos: (0, 0),
            scroll_offset: 0,
            selecting: false,
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

    pub fn sel_start_pos(&self) -> (u16, u16) { self.sel_start_pos.clone() }
    pub fn set_sel_start_pos(&mut self, sel_start_pos: (u16, u16)) {
        self.sel_start_pos = sel_start_pos;
    }

    pub fn sel_end_pos(&self) -> (u16, u16) { self.sel_end_pos.clone() }
    pub fn set_sel_end_pos(&mut self, sel_end_pos: (u16, u16)) {
        self.sel_end_pos = sel_end_pos;
    }

    /// The length of current line.
    fn current_line_len(&self) -> usize {
        match self.text.get(self.line_number as usize) {
            Some (line) => line.len(),
            None => 0,
        }
    }

    /// Counts the characters to the first whitespace character to the right of
    /// cursor.
    /// Ignores whitespace in current cursor position.
    fn next_whitespace_len(&self) -> u16 {
        if self.cursor_pos as usize >= self.current_line_len() {
            return 0;
        }

        let mut count: u16 = 1;

        let s = &self.text[self.line_number as usize];

        while self.cursor_pos + count < self.current_line_len() as u16 {
            match s.chars().nth((self.cursor_pos + count) as usize) {
                Some (c) => {
                    if c.is_whitespace() {
                        return count;
                    }

                    count += 1
                }
                None => { return count; }
            }
        }

        count -= 1;

        return count;
    }

    /// Counts the characters to the first whitespace character to the left of
    /// cursor.
    /// Ignores whitespace in current cursor position.
    /// if the flag `ignore_ws_imm_left` is `true` then ignores a whitespace
    /// immediately left of the current cursor position.
    fn prev_whitespace_len(&self, ignore_ws_imm_left: bool) -> u16 {
        if self.cursor_pos as usize == 0 {
            return 0;
        }

        let mut count: u16 = 1;

        let s = &self.text[self.line_number as usize];

        if ignore_ws_imm_left {
            match s.chars().nth((self.cursor_pos - 1) as usize) {
                Some (c) => {
                    if c.is_whitespace() && (self.cursor_pos - 1) > 0 {
                        count = 2;
                    }
                }
                None => {}
            }
        }

        while self.cursor_pos - count > 0 {
            match s.chars().nth((self.cursor_pos - count) as usize) {
                Some (c) => {
                    if c.is_whitespace() {
                        return count;
                    }

                    count += 1
                }
                None => { return count; }
            }
        }

        //count -= 1;

        return count;
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
                if self.line_number > 0 {
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
    }

    pub fn delete_word(&mut self) {
        let ws_len = self.prev_whitespace_len(true) as usize;

        let cursor_pos = self.cursor_pos as usize;

        if ws_len == cursor_pos {
            match self.text.get_mut(self.line_number as usize) {
                Some (s) => {
                    s.replace_range(0..cursor_pos, "");
                    self.cursor_pos = 0;
                }
                None => {}
            }
        }

        if ws_len < cursor_pos {
            match self.text.get_mut(self.line_number as usize) {
                Some (s) => {
                    let range_start = cursor_pos - ws_len + 1;

                    s.replace_range(range_start..cursor_pos, "");

                    self.cursor_pos = range_start as u16;
                }
                None => {}
            }
        }
    }

    pub fn delete_char_to_right(&mut self) {
        if self.cursor_pos < self.current_line_len() as u16 {
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
        } else {
            let line_num = self.line_number as usize;

            if line_num < self.text.len() - 1 {
                let mut current_line = self.text[line_num].clone();
                let next_line = self.text[line_num + 1].clone();
                
                current_line.push_str(next_line.as_str());

                self.text.remove(line_num);
                self.text.remove(line_num);

                self.text.insert(line_num, current_line);
            }
        }
    }

    pub fn delete_word_to_right(&mut self) {
        let ws_len = self.next_whitespace_len() as usize;

        if ws_len > 0 {
            match self.text.get_mut(self.line_number as usize) {
                Some (s) => {
                    let range_start = self.cursor_pos as usize;
                    let mut range_end = range_start + ws_len + 1;

                    if range_end <= s.len() {
                        s.replace_range(range_start..range_end, "");
                    } else {
                        range_end -= 1;
                        s.replace_range(range_start..range_end, "");
                    }
                }
                None => {}
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

    pub fn selecting(&self) -> bool { self.selecting.clone() }
    pub fn set_selecting(&mut self, selecting: bool) {
        self.selecting = selecting;
    }
    pub fn reset_selection(&mut self) {
        self.selecting = false;
        self.sel_start_pos = (0,0);
        self.sel_end_pos = (0,0);
    }

    pub fn move_cursor(
        &mut self,
        move_direction: TextEditMoveDirection,
        word: bool,
        select: bool
    ) {
        match move_direction {
            TextEditMoveDirection::Up => {
                if self.line_number > 0 {
                    self.line_number -= 1;

                    let line_len = self.current_line_len() as u16;

                    if self.cursor_pos > line_len {
                        self.cursor_pos = line_len;
                    }

                    if select {
                        if self.selecting() {
                            if self.cur_before_start() {
                                if self.sel_start_pos.0 == self.sel_end_pos.0
                                   && self.cursor_pos > self.sel_start_pos.1
                                {
                                    self.sel_end_pos = self.sel_start_pos;
                                    self.sel_start_pos = (self.line_number, self.cursor_pos);
                                } else {
                                    if self.line_number == self.sel_start_pos.0
                                        && self.cursor_pos == self.sel_end_pos.1
                                    {
                                        self.sel_end_pos = self.sel_start_pos;
                                        self.sel_start_pos = (self.line_number, self.cursor_pos);
                                    } else {
                                        self.sel_start_pos.0 = self.line_number;
                                    }
                                }
                            } else if self.cur_bet_start_and_end() {
                                self.sel_end_pos.0 = self.line_number;
                            } else {
                                self.sel_start_pos.0 = self.line_number;
                                self.sel_end_pos.0 = self.line_number;
                            }
                        } else {
                            self.selecting = true;
                            self.sel_start_pos = (self.line_number, self.cursor_pos);
                            self.sel_end_pos = (self.line_number + 1, self.cursor_pos);
                        }
                    } else {
                        self.reset_selection();
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

                    if select {
                        if self.selecting() {
                            if self.cur_bet_start_and_end() {
                                self.sel_start_pos.0 = self.line_number;
                            } else if self.cur_after_end() {
                                if self.line_number == self.sel_end_pos.0
                                    && self.cursor_pos > self.sel_end_pos.1
                                {
                                    self.sel_start_pos = self.sel_end_pos;
                                    self.sel_end_pos = (self.line_number, self.cursor_pos);
                                } else {
                                    self.sel_end_pos.0 = self.line_number;
                                }
                            } else {
                                self.sel_end_pos.0 = self.line_number;
                                self.sel_start_pos.0 = self.line_number;
                                self.sel_end_pos.0 = self.line_number;
                            }
                        } else {
                            self.selecting = true;
                            self.sel_start_pos = (self.line_number - 1, self.cursor_pos);
                            self.sel_end_pos = (self.line_number, self.cursor_pos);
                        }
                    } else {
                        self.reset_selection();
                    }
                }
            }

            TextEditMoveDirection::Right => {
                let end: u16 = self.current_line_len() as u16;

                if self.cursor_pos < end {
                    if word {
                        let nw = self.next_whitespace_len();

                        if ((nw + 1) as usize) < self.current_line_len() {
                            self.cursor_pos += nw + 1;
                        } else {
                            self.cursor_pos += nw;
                        }
                    } else {
                        self.cursor_pos += 1;
                    }

                    if select {
                        if self.selecting() {
                            if self.cur_bet_start_and_end() {
                                self.sel_start_pos.1 = self.cursor_pos;
                            } else if self.cur_after_end() {
                                self.sel_end_pos.1 = self.cursor_pos;
                            } else {
                                self.sel_end_pos.1 = self.cursor_pos;
                                self.sel_start_pos.1 = self.cursor_pos;
                            }
                        } else {
                            self.selecting = true;
                            self.sel_start_pos = (self.line_number, self.cursor_pos-1);
                            self.sel_end_pos = (self.line_number, self.cursor_pos);
                        }
                    } else {
                        self.reset_selection();
                    }
                }
            }

            TextEditMoveDirection::Left => {
                if self.cursor_pos > 0 {
                    if word {
                        let pw = self.prev_whitespace_len(true);

                        if self.cursor_pos - pw == 0 {
                            self.cursor_pos = 0;
                        } else {
                            self.cursor_pos -= pw - 1;
                        }
                    } else {
                        self.cursor_pos -= 1;
                    }

                    if select {
                        if self.selecting() {
                            if self.cur_before_start() {
                                self.sel_start_pos.1 = self.cursor_pos;
                            } else if self.cur_bet_start_and_end() {
                                self.sel_end_pos.1 = self.cursor_pos;
                            } else {
                                self.sel_end_pos.1 = self.cursor_pos;
                                self.sel_start_pos.1 = self.cursor_pos;
                            }
                        } else {
                            self.selecting = true;
                            self.sel_start_pos = (self.line_number, self.cursor_pos);
                            self.sel_end_pos = (self.line_number, self.cursor_pos+1);
                        }
                    } else {
                        self.reset_selection();
                    }
                }
            }

            TextEditMoveDirection::End => {
                let prev_cursor_pos = self.cursor_pos;
                self.cursor_pos = self.current_line_len() as u16;

                if select {
                    if self.sel_start_pos.0 == self.line_number {
                        self.sel_start_pos.1 = self.cursor_pos;
                    } else if self.sel_end_pos.0 == self.line_number {
                        self.sel_end_pos.1 = self.cursor_pos;
                    } else {
                        self.sel_start_pos = (self.line_number, prev_cursor_pos);
                        self.sel_end_pos = (self.line_number, self.cursor_pos);
                    }
                } else {
                    self.reset_selection();
                }
            }

            TextEditMoveDirection::Home => {
                let prev_cursor_pos = self.cursor_pos;
                self.cursor_pos = 0;

                if select {
                    if self.sel_start_pos.0 == self.line_number {
                        self.sel_start_pos.1 = self.cursor_pos;
                    } else if self.sel_end_pos.0 == self.line_number {
                        self.sel_end_pos.1 = self.cursor_pos;
                    } else {
                        self.sel_start_pos = (self.line_number, self.cursor_pos);
                        self.sel_end_pos = (self.line_number, prev_cursor_pos);
                    }
                } else {
                    self.reset_selection();
                }
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

    /// Is cursor cursor is before start
    fn cur_before_start(&self) -> bool {
        (
            self.sel_end_pos.0 == self.line_number
            && self.sel_start_pos.0 == self.line_number
            && self.cursor_pos < self.sel_start_pos.1
        )
        || (
            self.line_number == self.sel_start_pos.0
            && self.line_number < self.sel_end_pos.0
            && self.cursor_pos < self.sel_start_pos.1
        )
        || self.line_number < self.sel_start_pos.0
    }

    /// Is cursor between start and end
    fn cur_bet_start_and_end(&self) -> bool {
        (
            self.sel_end_pos.0 == self.line_number
            && self.sel_start_pos.0 == self.line_number
            && self.cursor_pos > self.sel_start_pos.1
            && self.cursor_pos < self.sel_end_pos.1
        )
        || (
            self.line_number == self.sel_start_pos.0
            && self.line_number < self.sel_end_pos.0
            && self.cursor_pos > self.sel_start_pos.1
        )
        || (
            self.line_number == self.sel_end_pos.0
            && self.line_number > self.sel_start_pos.0
            && self.cursor_pos < self.sel_end_pos.1
        )
        || (
            self.line_number > self.sel_start_pos.0
            && self.line_number < self.sel_end_pos.0
        )
    }

    /// Is cursor after end
    fn cur_after_end(&self) -> bool {
        (
            self.sel_end_pos.0 == self.line_number
            && self.cursor_pos > self.sel_end_pos.1
        )
        || self.line_number > self.sel_end_pos.0
    }
}

