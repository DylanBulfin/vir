use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::actions::{self, EditorAction, InsertAction};
use crate::editor::{EditorState, TextObject};

pub fn process_insert_input(ke: KeyEvent, state: &mut EditorState) -> Result<EditorAction> {
    let action = parse_insert_input(ke, state)?;
    let cursor_pos = state.cursor().pos();

    match action {
        InsertAction::Write(c) => state.insert_text(cursor_pos, &c.to_string()),
        InsertAction::DelForw => state.delete(TextObject::Char(cursor_pos)),
        InsertAction::DelBack => {
            if cursor_pos.index() == 0 {
                if cursor_pos.lnum() != 0 {
                    state.delete_newline(cursor_pos.lnum() - 1)
                }
            } else {
                state.backspace(cursor_pos)
            }
        }
        InsertAction::NewLine => state.insert_newline(),
        InsertAction::Indent => state.indent(),
        InsertAction::Up => state.cursor_up(),
        InsertAction::Down => state.cursor_down(),
        InsertAction::Left => state.cursor_left(),
        InsertAction::Right => state.cursor_right(),
        InsertAction::NormalMode => state.normal_mode(),
        InsertAction::None => (),
    }

    Ok(EditorAction::None)
}

fn parse_insert_input(ke: KeyEvent, state: &mut EditorState) -> Result<InsertAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            let action = match ke.code {
                KeyCode::Char(c) => Some(InsertAction::Write(c)),
                _ => state.insert_bind(&actions::get_key_name(&ke)),
            };

            match action {
                Some(a) => a,
                None => InsertAction::None,
            }
        } else {
            InsertAction::None
        },
    )
}
