use crossterm::event::{Event, KeyEvent};

use crate::{editor::TextObject, terminal::Term};

pub(crate) enum VisualAction {
    Delete(TextObject),
    Change(TextObject),
    Yank(TextObject),

    NormalMode,
}


