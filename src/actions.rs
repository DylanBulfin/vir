use crossterm::event::{Event, KeyEvent};

use crate::{editor::TextObject, term::Term};

pub enum EditorAction {
    None,
    Exit,
}

#[derive(Clone, Copy)]
pub(crate) enum InsertAction {
    Write(char),
    DelForw,
    DelBack,
    NewLine,
    Indent,

    Up,
    Down,
    Left,
    Right,

    NormalMode,

    None,
}

#[derive(Clone, Copy)]
pub(crate) enum NormalAction {
    ReplaceChar,
    Delete,
    Change,
    Yank,

    LineStart,
    LineEnd,

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

#[derive(Clone, Copy)]
pub(crate) enum VisualAction {
    ReplaceChar,
    Delete,
    Change,
    Yank,

    Up,
    Down,
    Left,
    Right,

    NormalMode,
}
