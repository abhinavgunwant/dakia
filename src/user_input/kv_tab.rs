use crossterm::event::{ KeyCode, KeyModifiers, KeyEvent };

/// Responsible for different operations performed in a key-value tab.
pub enum KVTabOperation {
    /// Inserts a key-value pair at the position
    Insert(u16),
    Remove(u16),
    MoveColumn(u8),
    MoveRow(u16),
    AppendText(char),
    PopText(),
}

pub fn process_kv_tab_input(
    key: KeyEvent,
    row: u16,
    col: u8,
    mut update_func: impl FnMut(KVTabOperation),
    mut change_func: impl FnMut(bool),
) {
    // Whether control key is pressed
    let ctrl_down = key.modifiers == KeyModifiers::CONTROL;

    match key.code {
        KeyCode::Enter => {
            if col == 2 {
                update_func(KVTabOperation::Insert(row+1));
                change_func(true);
            }

            // The "-" or remove parameter button
            if col == 3 {
                if row > 0 {
                    update_func(KVTabOperation::MoveRow(row-1));
                }

                update_func(KVTabOperation::Remove(row));
                change_func(true);
            }
        }

        KeyCode::Up => {
            if ctrl_down {
                if row != 0 {
                    update_func(KVTabOperation::MoveRow(row - 1));
                }
            }
        }

        KeyCode::Right => {
            if ctrl_down {
                if col < 3 {
                    update_func(KVTabOperation::MoveColumn(col + 1));
                }
            }
        }

        KeyCode::Left => {
            if ctrl_down {
                if col > 0 {
                    update_func(KVTabOperation::MoveColumn(col - 1));
                }
            }
        }

        KeyCode::Down => {
            if ctrl_down {
                if row < 1000 {
                    update_func(KVTabOperation::MoveRow(row + 1));
                }
            }
        }


        KeyCode::Char(c) => {
            update_func(KVTabOperation::AppendText(c));
            change_func(true);
        }

        KeyCode::Backspace => {
            update_func(KVTabOperation::PopText());
            change_func(true);
        }

        _ => {}
    }
}

