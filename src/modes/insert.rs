use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::Buffer;

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

const TABSTOP: u32 = 2;

pub fn process_insert_input(ke: KeyEvent, buf: &mut Buffer) -> Result<()> {
    let action = parse_insert_input(ke)?;

    match action {
        InsertAction::Write(c) => todo!(),
        InsertAction::DelForw => todo!(),
        InsertAction::DelBack => todo!(),
        InsertAction::NewLine => todo!(),
        InsertAction::Indent => todo!(),
        InsertAction::Up => todo!(),
        InsertAction::Down => todo!(),
        InsertAction::Left => todo!(),
        InsertAction::Right => todo!(),
        InsertAction::NormalMode => todo!(),
        InsertAction::Exit => todo!(),
        InsertAction::None => todo!(),
    }
}


fn parse_insert_input(ke: KeyEvent) -> Result<InsertAction> {
    Ok(
        if ke.modifiers == KeyModifiers::from_name("NONE").expect("Unable to check modifiers") {
            match ke.code {
                KeyCode::Backspace => InsertAction::DelBack,
                KeyCode::Enter => InsertAction::NewLine,
                KeyCode::Left => InsertAction::Left,
                KeyCode::Right => InsertAction::Right,
                KeyCode::Up => InsertAction::Up,
                KeyCode::Down => InsertAction::Down,
                KeyCode::Tab => InsertAction::Indent,
                KeyCode::Delete => InsertAction::DelForw,
                KeyCode::Esc => InsertAction::NormalMode,
                KeyCode::Char(_) => todo!(),
                _ => InsertAction::None,
            }
        } else if ke.modifiers
            == KeyModifiers::from_name("CONTROL").expect("Unable to check modifiers")
        {
            if let KeyCode::Char(c) = ke.code {}
        } else {
            InsertAction::None
        },
    )
}
