use crossterm::event::{KeyEvent, KeyModifiers};
use std::io::Result;

use crate::{
    actions::{self, EditorAction, NormalAction},
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
        NormalAction::Delete => state.delete(pending::await_textobject(state, ke)?),
        NormalAction::DeleteChar => state.delete(TextObject::Char(cursor_pos)),
        NormalAction::Change => {
            state.delete(pending::await_textobject(state, ke)?);
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
            let action = state.normal_bind(&actions::get_key_name(&ke));

            match action {
                Some(a) => a,
                None => NormalAction::None,
            }
        } else {
            NormalAction::None
        },
    )
}
