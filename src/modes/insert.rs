use std::{default, io::Result};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::{EditorState, Position, TextObject};
use crate::actions::{InsertAction, EditorAction};

const TABSTOP: u32 = 2;

pub fn process_insert_input(ke: KeyEvent, buf: &mut EditorState) -> Result<EditorAction> {
    let action = parse_insert_input(ke)?;
    let cursor_pos = buf.cursor_pos();

    match action {
        InsertAction::Write(c) => buf.insert(cursor_pos, &c.to_string()),
        InsertAction::DelForw => buf.delete(TextObject::Char(cursor_pos)),
        InsertAction::DelBack => {
            if cursor_pos.index() == 0 {
                if cursor_pos.lnum() != 0 {
                    buf.delete_newline(cursor_pos.lnum() - 1)
                }
            } else {
                buf.backspace(cursor_pos)
            }
        }
        InsertAction::NewLine => buf.insert_newline(cursor_pos),
        InsertAction::Indent => todo!(),
        InsertAction::Up => buf.cursor_up(),
        InsertAction::Down => buf.cursor_down(),
        InsertAction::Left => buf.cursor_left(),
        InsertAction::Right => buf.cursor_right(),
        InsertAction::NormalMode => buf.normal_mode(),
        InsertAction::Exit => return Ok(EditorAction::Exit),
        InsertAction::None => (),
    }

    Ok(EditorAction::None)
}

fn parse_insert_input(ke: KeyEvent) -> Result<InsertAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            match ke.code {
                KeyCode::Backspace => InsertAction::DelBack,
                KeyCode::Enter => InsertAction::NewLine,
                KeyCode::Left => InsertAction::Left,
                KeyCode::Right => InsertAction::Right,
                KeyCode::Up => InsertAction::Up,
                KeyCode::Down => InsertAction::Down,
                KeyCode::Tab => InsertAction::Indent,
                KeyCode::Delete => InsertAction::DelForw,
                KeyCode::Esc => InsertAction::NormalMode,
                KeyCode::Char(c) => InsertAction::Write(c),
                _ => InsertAction::None,
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
