use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use insert::process_insert_input;
use normal::process_normal_input;

use crate::{actions::EditorAction, editor::EditorState};

mod insert;
mod normal;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Normal,
    Visual,
}

pub fn process_key_event(ke: KeyEvent, buf: &mut EditorState) -> Result<EditorAction>{
    match buf.mode() {
        Mode::Insert => process_insert_input(ke, buf),
        Mode::Normal => process_normal_input(ke, buf),
        Mode::Visual => todo!(),
    }
}