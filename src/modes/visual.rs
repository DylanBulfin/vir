use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;

use crate::{
    actions::{EditorAction, VisualAction},
    editor::{EditorState, TextObject},
};

use super::pending::await_char;

pub fn process_visual_input(ke: KeyEvent, state: &mut EditorState) -> Result<EditorAction> {
    let action = parse_visual_input(ke, state)?;
    let cursor_pos = state.cursor().pos();

    match action {
        VisualAction::ReplaceChar => {
            state.replace(TextObject::Char(cursor_pos), &await_char()?.to_string())
        }
        VisualAction::Delete => state.delete(TextObject::Selection(cursor_pos)),
        VisualAction::DeleteChar => state.delete(TextObject::Char(cursor_pos)),
        VisualAction::Change => {
            state.delete(TextObject::Selection(cursor_pos));
            state.insert_mode();
        }
        VisualAction::Yank => todo!(),
        VisualAction::LineStart => state.line_start(),
        VisualAction::LineEnd => state.line_end(),
        VisualAction::Up => state.cursor_up(),
        VisualAction::Down => state.cursor_down(),
        VisualAction::Left => state.cursor_left(),
        VisualAction::Right => state.cursor_right(),
        VisualAction::NormalMode => state.normal_mode(),
        VisualAction::None => return Ok(EditorAction::None),
        VisualAction::Exit => return Ok(EditorAction::Exit),
    }

    Ok(EditorAction::None)
}

fn parse_visual_input(ke: KeyEvent, state: &mut EditorState) -> Result<VisualAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            let action = match ke.code {
                KeyCode::Backspace => state.visual_bind("backspace"),
                KeyCode::Enter => state.visual_bind("enter"),
                KeyCode::Left => state.visual_bind("left"),
                KeyCode::Right => state.visual_bind("right"),
                KeyCode::Up => state.visual_bind("up"),
                KeyCode::Down => state.visual_bind("down"),
                KeyCode::Tab => state.visual_bind("tab"),
                KeyCode::Delete => state.visual_bind("delete"),
                KeyCode::Esc => state.visual_bind("esc"),
                KeyCode::Char(c) => state.visual_bind(&c.to_string()),
                _ => Some(VisualAction::None),
            };

            match action {
                Some(a) => a,
                None => VisualAction::None,
            }
        } else {
            VisualAction::None
        },
    )
}
