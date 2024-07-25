use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::editor::{EditorState, TextObject};
pub(crate) fn await_char() -> Result<char> {
    loop {
        let c = match event::read()? {
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

pub(crate) fn await_textobject(state: &EditorState) -> Result<TextObject> {
    loop {
        let textobject = match event::read()? {
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
                TextObject::LineEnd(_, _) => state.get_lineend_textobject(pos),
                TextObject::Word(_, _) => state.get_word_textobject(pos),
                TextObject::None => panic!("Shouldn't see None here"),
                TextObject::Selection(_) => panic!("Shouldn't see Selection here"),
            });
        }
    }
}