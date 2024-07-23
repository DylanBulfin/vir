use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;

use crate::editor::{EditorAction, EditorState, TextObject};

use super::insert::InsertAction;

pub(crate) enum NormalAction {
    ReplaceChar(char),
    Delete(TextObject),
    Change(TextObject),
    Yank(TextObject),

    Up,
    Down,
    Left,
    Right,

    SearchMode,
    InsertMode,
    VisualMode,
    Exit,

    None,
}

pub fn process_normal_input(ke: KeyEvent, buf: &mut EditorState) -> Result<EditorAction> {
    let action = parse_normal_input(ke)?;
    let cursor_pos = buf.cursor_pos();

    match action {
        NormalAction::ReplaceChar(_) => todo!(),
        NormalAction::Delete(_) => todo!(),
        NormalAction::Change(_) => todo!(),
        NormalAction::Yank(_) => todo!(),
        NormalAction::SearchMode => todo!(),
        NormalAction::InsertMode => buf.insert_mode(),
        NormalAction::VisualMode => todo!(),
        NormalAction::Exit => todo!(),
        NormalAction::None => todo!(),
        NormalAction::Up => buf.cursor_up(),
        NormalAction::Down => buf.cursor_down(),
        NormalAction::Left => buf.cursor_left(),
        NormalAction::Right => buf.cursor_right(),
    }

    Ok(EditorAction::None)
}

fn parse_char_input(c: char) -> Result<NormalAction> {
    match c {
        'm' => Ok(NormalAction::Left),
        'n' => Ok(NormalAction::Down),
        'e' => Ok(NormalAction::Up),
        'i' => Ok(NormalAction::Right),
        'u' => Ok(NormalAction::InsertMode),
        _ => Ok(NormalAction::None),
    }
}

fn parse_normal_input(ke: KeyEvent) -> Result<NormalAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            match ke.code {
                KeyCode::Char(c) => parse_char_input(c)?,
                _ => NormalAction::None,
            }
        } else if ke.modifiers
            == KeyModifiers::from_name("CONTROL").expect("Unable to check modifiers")
        {
            if let KeyCode::Char(c) = ke.code {
                if c == 'c' {
                    NormalAction::Exit
                } else {
                    NormalAction::None
                }
            } else {
                NormalAction::None
            }
        } else {
            NormalAction::None
        },
    )
}
