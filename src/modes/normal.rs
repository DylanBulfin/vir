use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;

use crate::{
    actions::{EditorAction, NormalAction},
    editor::{EditorState, TextObject},
};

use super::pending;

pub fn process_normal_input(ke: KeyEvent, state: &mut EditorState) -> Result<EditorAction> {
    let action = parse_normal_input(ke, state)?;
    let cursor_pos = state.cursor().pos();

    match action {
        NormalAction::ReplaceChar => state.replace(
            TextObject::Char(cursor_pos),
            &pending::await_char()?.to_string(),
        ),
        NormalAction::Delete => state.delete(pending::await_textobject(state)?),
        NormalAction::DeleteChar => state.delete(TextObject::Char(cursor_pos)),
        NormalAction::Change => {
            state.delete(pending::await_textobject(state)?);
            state.insert_mode()
        }
        NormalAction::Yank => todo!(),
        NormalAction::SearchMode => todo!(),
        NormalAction::InsertMode => state.insert_mode(),
        NormalAction::VisualMode => state.visual_mode(),
        NormalAction::Exit => return Ok(EditorAction::Exit),
        NormalAction::None => return Ok(EditorAction::None),
        NormalAction::Up => state.cursor_up(),
        NormalAction::Down => state.cursor_down(),
        NormalAction::Left => state.cursor_left(),
        NormalAction::Right => state.cursor_right(),
        NormalAction::LineStart => state.line_start(),
        NormalAction::LineEnd => state.line_end(),
    }

    Ok(EditorAction::None)
}

fn parse_normal_input(ke: KeyEvent, state: &EditorState) -> Result<NormalAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            let action = match ke.code {
                KeyCode::Backspace => state.normal_bind("backspace"),
                KeyCode::Enter => state.normal_bind("enter"),
                KeyCode::Left => state.normal_bind("left"),
                KeyCode::Right => state.normal_bind("right"),
                KeyCode::Up => state.normal_bind("up"),
                KeyCode::Down => state.normal_bind("down"),
                KeyCode::Tab => state.normal_bind("tab"),
                KeyCode::Delete => state.normal_bind("delete"),
                KeyCode::Esc => state.normal_bind("esc"),
                KeyCode::Char(c) => state.normal_bind(&c.to_string()),
                _ => Some(NormalAction::None),
            };

            match action {
                Some(a) => a,
                None => NormalAction::None,
            }
        } else {
            NormalAction::None
        },
    )
}
