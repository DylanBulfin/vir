use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use insert::process_insert_input;

use crate::editor::{EditorAction, EditorState};

mod insert;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Normal,
    Visual,
}

pub fn process_key_event(ke: KeyEvent, buf: &mut EditorState) -> Result<EditorAction>{
    match buf.mode() {
        Mode::Insert => process_insert_input(ke, buf),
        Mode::Normal => todo!(),
        Mode::Visual => todo!(),
    }
}