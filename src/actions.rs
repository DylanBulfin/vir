use crossterm::event::{Event, KeyEvent};

use crate::{editor::TextObject, terminal::Term};

pub enum EditorAction {
    None,
    Exit,
}


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
    Exit,

    None,
}

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

pub(crate) enum VisualAction {
    ReplaceChar(char),
    Delete(TextObject),
    Change(TextObject),
    Yank(TextObject),

    Up,
    Down,
    Left,
    Right,

    NormalMode,
}

