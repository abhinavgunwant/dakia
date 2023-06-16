//! The state associated to the multi-line TextInput widget.

use log::info;
use copypasta::{ ClipboardContext, ClipboardProvider };

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
    content_height: u16,
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
            content_height: 0,
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

    /// Returns line at `line_number` index.
    fn line(&self, line_number: u16) -> String {
        match self.text.get(line_number as usize) {
            Some (line) => line.clone(),
            None => String::default(),
        }
    }

    /// Gets the current line.
    fn current_line(&self) -> String {
        self.line(self.line_number)
    }

    /// Length of the `line_number`th line
    fn line_len(&self, line_number: u16) -> usize {
        self.line(line_number).len()
    }

    /// The length of current line.
    fn current_line_len(&self) -> usize {
        self.line_len(self.line_number)
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

                    if self.scroll_offset + self.content_height > self.text.len() as u16
                        && self.scroll_offset > 0
                    {
                        self.scroll_offset -= 1;
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

    fn delete_first_and_last_lines_from_selection(&mut self) {
        if self.sel_start_pos.1 == 0 {
            let end_line_len = self.line_len(self.sel_end_pos.0);

            match self.text.get_mut(self.sel_end_pos.0 as usize) {
                Some (s) => {
                    let mut end_char_pos = (self.sel_end_pos.1 + 1) as usize;

                    if end_char_pos >= end_line_len {
                        end_char_pos = end_line_len;
                    }

                    s.replace_range(0..end_char_pos, "");
                }
                None => {}
            }

            self.text.remove(self.sel_start_pos.0 as usize);
        } else {
            let last_line_len = self.line_len(self.sel_start_pos.0);

            let last_char_pos = (self.sel_end_pos.1 + 1) as usize;

            let mut last_line = self.line(self.sel_end_pos.0);
            last_line.replace_range(0..last_char_pos, "");

            match self.text.get_mut(self.sel_start_pos.0 as usize) {
                Some (s) => {
                    s.replace_range(
                        (self.sel_start_pos.1 as usize)
                        ..last_line_len,
                        ""
                    );

                    s.push_str(last_line.as_str());
                }
                None => {}
            }

            self.text.remove(self.sel_end_pos.0 as usize);
        }
    }

    pub fn delete_selected(&mut self) {
        let line_diff = self.sel_end_pos.0 - self.sel_start_pos.0;

        match line_diff {
            0 => {
                let start_line_len = self.line_len(self.sel_start_pos.0);

                match self.text.get_mut(self.sel_start_pos.0 as usize) {
                    Some (s) => {
                        let mut end_char_pos = (self.sel_end_pos.1 + 1) as usize;

                        if end_char_pos > start_line_len {
                            end_char_pos = start_line_len;
                        }

                        s.replace_range(
                            (self.sel_start_pos.1 as usize)
                            ..end_char_pos,
                            ""
                        );
                    }
                    None => {}
                }
            }

            1 => {
                self.delete_first_and_last_lines_from_selection();
            }

            _ => {
                let line_to_delete = self.sel_start_pos.0 + 1;

                for _ in line_to_delete..self.sel_end_pos.0 {
                    self.text.remove(line_to_delete as usize);
                    self.sel_end_pos.0 -= 1;
                }

                self.delete_first_and_last_lines_from_selection();
            }
        }

        self.line_number = self.sel_start_pos.0;
        self.cursor_pos = self.sel_start_pos.1;

        self.reset_selection();
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.selecting() {
            self.delete_selected();
            self.reset_selection();
        }

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

    pub fn content_height(&self) -> u16 { self.content_height.clone() }
    pub fn set_content_height(&mut self, content_height: u16) {
        self.content_height = content_height;
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

                    if self.line_number < self.scroll_offset {
                        self.scroll_offset = self.line_number;
                    }
                }
            }

            TextEditMoveDirection::Down => {
                if self.line_number + 1 < self.text.len() as u16 {
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
                                }

                                self.sel_end_pos = (self.line_number, self.cursor_pos);
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

                    info!("scroll_offset: {}, content_height: {}, line_number: {}",
                        self.scroll_offset,
                        self.content_height,
                        self.line_number);

                    if self.line_number >= self.scroll_offset + self.content_height {
                        self.scroll_offset = self.line_number - self.content_height + 1;
                    }
                }
            }

            TextEditMoveDirection::Right => {
                let end: u16 = self.current_line_len() as u16;
                let previous_pos = self.cursor_pos;

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

                            if word {
                                self.sel_start_pos = (self.line_number, previous_pos);
                            } else {
                                self.sel_start_pos = (self.line_number, self.cursor_pos-1);
                            }

                            self.sel_end_pos = (self.line_number, self.cursor_pos);
                        }
                    } else {
                        self.reset_selection();
                    }
                }
            }

            TextEditMoveDirection::Left => {
                let previous_pos = self.cursor_pos;

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

                            if word {
                                self.sel_end_pos = (self.line_number, previous_pos);
                            } else {
                                self.sel_end_pos = (self.line_number, self.cursor_pos+1);
                            }

                            self.sel_start_pos = (self.line_number, self.cursor_pos);
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
                    if self.selecting() {
                        if self.sel_start_pos.0 == self.line_number
                            && self.sel_end_pos.0 == self.line_number
                        {
                            self.sel_start_pos.1 = self.sel_end_pos.1;
                            self.sel_end_pos.1 = self.cursor_pos;
                        } else if self.sel_start_pos.0 == self.line_number {
                            self.sel_start_pos.1 = self.cursor_pos;
                        } else if self.sel_end_pos.0 == self.line_number {
                            self.sel_end_pos.1 = self.cursor_pos;
                        } else {
                            self.sel_end_pos = (self.line_number, self.cursor_pos);
                        }
                    } else {
                        self.selecting = true;
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
                    if self.selecting() {
                        if self.sel_start_pos.0 == self.line_number
                            && self.sel_end_pos.0 == self.line_number
                        {
                            self.sel_end_pos.1 = self.sel_start_pos.1;
                            self.sel_start_pos.1 = 0;
                        } else if self.sel_start_pos.0 == self.line_number {
                            self.sel_start_pos.1 = self.cursor_pos;
                        // } else if self.sel_end_pos.0 == self.line_number {
                        } else {
                            self.sel_end_pos.1 = self.cursor_pos;
                        }
                    } else {
                        self.selecting = true;

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
        let s = self.current_line();

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

        info!("scroll_offset: {}, content_height: {}, line_number: {}",
            self.scroll_offset,
            self.content_height,
            self.line_number);

        if self.line_number >= self.scroll_offset + self.content_height {
            self.scroll_offset = self.line_number - self.content_height + 1;
        }
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

    pub fn select_all(&mut self) {
        let last_line_number = (self.text.len() - 1) as u16;
        let last_line_len = self.line_len(last_line_number) as u16;

        self.selecting = true;
        self.sel_start_pos = (0, 0);
        self.sel_end_pos = (last_line_number, last_line_len);

        self.line_number = last_line_number;
        self.cursor_pos = last_line_len;
    }

    pub fn selected_text(&self) -> String {
        let line_diff = self.sel_end_pos.0 - self.sel_start_pos.0;
        let first_line = self.line(self.sel_start_pos.0);

        info!(
            "\n\tstart: ({}, {})\n\tend: ({}, {})",
            self.sel_start_pos.0,
            self.sel_start_pos.1,
            self.sel_end_pos.0,
            self.sel_end_pos.1,
        );

        match line_diff {
            0 => first_line.chars().skip(self.sel_start_pos.1 as usize)
                .take((self.sel_end_pos.1 - self.sel_start_pos.1 + 1) as usize)
                .collect(),

            1 => {
                let mut s = String::default();

                s.push_str(
                    first_line.chars().skip(self.sel_start_pos.1 as usize)
                        .take(self.line_len(self.sel_start_pos.0))
                        .collect::<String>()
                        .as_str()
                );

                s.push('\n');

                s.push_str(
                    self.line(self.sel_end_pos.0).chars().skip(0)
                        .take((self.sel_end_pos.1 + 1) as usize)
                        .collect::<String>()
                        .as_str()
                );

                s
            }

            _ => {
                let mut s = String::default();

                s.push_str(
                    first_line.chars().skip(self.sel_start_pos.1 as usize)
                        .take(self.line_len(self.sel_start_pos.0))
                        .collect::<String>()
                        .as_str()
                );

                for i in (self.sel_start_pos.0 + 1) .. self.sel_end_pos.0 {
                    s.push('\n');
                    s.push_str(self.line(i).as_str());
                }

                s.push('\n');

                s.push_str(
                    self.line(self.sel_end_pos.0).chars().skip(0)
                        .take((self.sel_end_pos.1 + 1) as usize)
                        .collect::<String>()
                        .as_str()
                );

                s
            }
        }
    }

    /// Copies the text selected in textarea to clipboard.
    pub fn copy_selected(&mut self) {
        if self.selecting() {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(self.selected_text()).unwrap();

            self.reset_selection();
        }
    }

    /// Pastes the text from clipboard to the text area.
    pub fn paste(&mut self) {
        if self.selecting() {
            self.delete_selected();
            self.reset_selection();
        }

        let mut ctx = ClipboardContext::new().unwrap();
        let s = ctx.get_contents().unwrap();
        let s_vec: Vec<String> = s.replace('\r', "").split('\n').map(str::to_owned).collect::<Vec<_>>();

        if s_vec.is_empty() {
            return;
        }

        // if no text entered in text area
        if self.text.len() == 1 && self.line_len(0) == 0 {
            self.text = s_vec;

            self.line_number = (self.text.len() - 1) as u16;
            self.cursor_pos = self.line_len(self.line_number) as u16;

            return;
        }

        let current_line = self.current_line();
        let (left, right) = current_line.split_at(self.cursor_pos as usize);

        self.text.remove(self.line_number as usize);

        let mut updated_line: String = String::from(left);
        updated_line.push_str(s_vec[0].as_str());

        match s_vec.len() {
            1 => {
                self.cursor_pos = updated_line.len() as u16;
                updated_line.push_str(right);

                self.text.insert(self.line_number as usize, updated_line);
            }

            _ => {
                self.text.insert(self.line_number as usize, updated_line);

                for copied_line in &s_vec[1..] {
                    self.line_number += 1;
                    self.text.insert(self.line_number as usize, String::from(copied_line));
                }

                if !right.is_empty() {
                    let mut last_line = self.current_line();
                    self.text.remove(self.line_number as usize);
                    self.cursor_pos = last_line.len() as u16;
                    last_line.push_str(right);
                    self.text.insert(self.line_number as usize, last_line);
                } else {
                    self.cursor_pos = self.current_line_len() as u16;
                }
            }
        }
    }
}

