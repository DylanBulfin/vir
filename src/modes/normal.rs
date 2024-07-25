use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;

use crate::{
    actions::{EditorAction, NormalAction},
    editor::{EditorState, TextObject},
};

pub fn process_normal_input(ke: KeyEvent, state: &mut EditorState) -> Result<EditorAction> {
    let action = parse_normal_input(ke, state)?;
    let cursor_pos = state.cursor().pos();

    match action {
        NormalAction::ReplaceChar => {
            state.replace(TextObject::Char(cursor_pos), &await_char()?.to_string())
        }
        NormalAction::Delete => state.delete(await_textobject(state)?),
        NormalAction::Change => {
            state.delete(await_textobject(state)?);
            state.insert_mode()
        }
        NormalAction::Yank => todo!(),
        NormalAction::SearchMode => todo!(),
        NormalAction::InsertMode => state.insert_mode(),
        NormalAction::VisualMode => todo!(),
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

fn await_char() -> Result<char> {
    loop {
        let c = match read()? {
            Event::Key(ke) => match ke.code {
                KeyCode::Char(c) => c,
                _ => continue,
            },
            Event::Resize(_, _) => panic!(),
            _ => continue,
        };

        return Ok(c);
    }
}

fn await_textobject(state: &EditorState) -> Result<TextObject> {
    loop {
        let textobject = match read()? {
            Event::Key(ke) => match ke.code {
                KeyCode::Backspace => state.textobject_bind("backspace"),
                KeyCode::Enter => state.textobject_bind("enter"),
                KeyCode::Left => state.textobject_bind("left"),
                KeyCode::Right => state.textobject_bind("right"),
                KeyCode::Up => state.textobject_bind("up"),
                KeyCode::Down => state.textobject_bind("down"),
                KeyCode::Tab => state.textobject_bind("tab"),
                KeyCode::Delete => state.textobject_bind("delete"),
                KeyCode::Char(c) => state.textobject_bind(&c.to_string()),
                KeyCode::Esc => state.textobject_bind("esc"),
                _ => continue,
            },
            Event::Resize(_, _) => panic!(),
            _ => continue,
        };

        if let Some(textobject) = textobject {
            let pos = state.cursor().pos();

            return Ok(match textobject {
                TextObject::CancelOp => TextObject::None,
                TextObject::Char(_) => TextObject::Char(pos),
                TextObject::Line(_) => TextObject::Line(pos.lnum()),
                TextObject::LineEnd(_) => TextObject::LineEnd(pos),
                TextObject::Word(_, _) => state.get_word_textobject(pos),
                TextObject::None | TextObject::Other(_, _) => panic!("Shouldn't see None here"),
            });
        }
    }
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
