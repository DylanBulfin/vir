use crossterm::event::{KeyCode, KeyEvent};

pub enum EditorAction {
    None,
    Exit,
    Save,
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
    DeleteChar,
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
    DeleteChar,
    Change,
    Yank,

    LineStart,
    LineEnd,

    Up,
    Down,
    Left,
    Right,

    NormalMode,
    Exit,

    None,
}

pub(crate) fn get_key_name(ke: &KeyEvent) -> String {
    match ke.code {
        KeyCode::Backspace => String::from("backspace"),
        KeyCode::Enter => String::from("enter"),
        KeyCode::Left => String::from("left"),
        KeyCode::Right => String::from("right"),
        KeyCode::Up => String::from("up"),
        KeyCode::Down => String::from("down"),
        KeyCode::Tab => String::from("tab"),
        KeyCode::Delete => String::from("delete"),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Esc => String::from("esc"),
        _ => panic!("Invalid key"),
    }
}
