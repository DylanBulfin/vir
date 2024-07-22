use std::io::Result;
use std::ops::Deref;

use crate::{terminal::Term, util};

#[derive(Clone, Copy)]
pub(crate) struct Position {
    lnum: usize,
    index: usize,
}

impl Position {
    pub fn new(lnum: usize, index: usize) -> Self {
        Self { lnum, index }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            lnum: value.0,
            index: value.1,
        }
    }
}

pub(crate) enum TextObject {
    Char(Position),      // Line number, index
    Line(usize),         // Line number
    LineEnd(Position),   // Line number, start_index
    Lines(usize, u16),   // Line number, count
    Word(Position, u16), // Line number, length

    // (start_lnum, start_index), (end_lnum, end_index)
    Other(Position, Position),
}

impl TextObject {
    pub fn get_start(&self) -> Position {
        match self {
            TextObject::Char(p)
            | TextObject::LineEnd(p)
            | TextObject::Word(p, _)
            | TextObject::Other(p, _) => *p,

            TextObject::Line(lnum) | TextObject::Lines(lnum, _) => Position::new(*lnum, 0),
        }
    }

    pub fn get_end(&self) -> Position {
        match self {
            TextObject::Char(p) => Position::new(p.lnum, p.index + 1),
            TextObject::Line(lnum) => Position::new(lnum + 1, 0),
            TextObject::LineEnd(p) => Position::new(p.lnum + 1, 0),
            TextObject::Lines(lnum, c) => Position::new(*lnum + *c as usize, 0),
            TextObject::Word(p, c) => Position::new(p.lnum, p.index + *c as usize),
            TextObject::Other(_, p) => *p,
        }
    }
}

pub(crate) struct Editor {
    data: Vec<String>,
    term_x: usize,
    term_y: usize,
}

impl Editor {
    pub fn new(data: Vec<String>) -> Self {
        Editor {
            data,
            term_x: 0,
            term_y: 0,
        }
    }

    pub fn get_slice(&self, height: usize) -> &'_ [String] {
        &self.data[self.term_y..self.term_y + height]
    }
}
