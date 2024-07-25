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
