use crossterm::event::{Event, KeyEvent};

use crate::{editor::TextObject, terminal::Term};

pub(crate) enum InsertAction {
    Write(char),
    DelForw,
    DelBack,
    NewLine,

    NormalMode,
}

pub(crate) enum NormalAction {
    ReplaceChar(char),
    Delete(TextObject),
    Change(TextObject),
    Yank(TextObject),

    SearchMode,
    InsertMode,
    VisualMode,
}

pub(crate) enum VisualAction {
    Delete(TextObject),
    Change(TextObject),
    Yank(TextObject),

    NormalMode,
}


pub fn process_insert_input(ke: KeyEvent, term: &mut Term) {}
