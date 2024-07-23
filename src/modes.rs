use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};

use crate::editor::Buffer;

mod insert;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Normal,
    Visual,
}

impl Mode {
    pub fn execute_action(&self, ed: &mut Buffer) -> Result<()> {
        match self {
            Mode::Insert => todo!(),
            Mode::Normal => todo!(),
            Mode::Visual => todo!(),
        }
    }
}
