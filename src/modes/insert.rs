use std::{default, io::Result};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::actions::{EditorAction, InsertAction};
use crate::editor::{EditorState, Position, TextObject};

const TABSTOP: u32 = 2;

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
        InsertAction::Exit => return Ok(EditorAction::Exit),
        InsertAction::None => (),
    }

    Ok(EditorAction::None)
}

fn parse_insert_input(ke: KeyEvent, state: &mut EditorState) -> Result<InsertAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            let action = match ke.code {
                KeyCode::Backspace => state.insert_bind("backspace"),
                KeyCode::Enter => state.insert_bind("enter"),
                KeyCode::Left => state.insert_bind("left"),
                KeyCode::Right => state.insert_bind("right"),
                KeyCode::Up => state.insert_bind("up"),
                KeyCode::Down => state.insert_bind("down"),
                KeyCode::Tab => state.insert_bind("tab"),
                KeyCode::Delete => state.insert_bind("delete"),
                KeyCode::Esc => state.insert_bind("esc"),
                KeyCode::Char(c) => Some(InsertAction::Write(c)),
                _ => Some(InsertAction::None),
            };

            match action {
                Some(a) => a,
                None => InsertAction::None,
            }
        } else if ke.modifiers
            == KeyModifiers::from_name("CONTROL").expect("Unable to check modifiers")
        {
            if let KeyCode::Char(c) = ke.code {
                if c == 'c' {
                    InsertAction::Exit
                } else {
                    InsertAction::None
                }
            } else {
                InsertAction::None
            }
        } else {
            InsertAction::None
        },
    )
}
