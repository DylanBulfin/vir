use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    actions,
    editor::{EditorState, TextObject},
};
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

pub(crate) fn await_textobject(state: &EditorState, prev: KeyEvent) -> Result<TextObject> {
    loop {
        let textobject = match event::read()? {
            Event::Key(ke) => {
                if actions::get_key_name(&ke) != actions::get_key_name(&prev) {
                    Some(TextObject::Line(0))
                } else {
                    state.textobject_bind(&actions::get_key_name(&ke))
                }
            }
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
