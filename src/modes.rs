use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use insert::process_insert_input;
use normal::process_normal_input;
use visual::process_visual_input;

use crate::{actions::EditorAction, editor::EditorState};

mod insert;
mod normal;
mod pending;
mod visual;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Normal,
    Visual,
}

impl Mode {
    pub fn get_name(&self) -> &str {
        match self {
            Mode::Insert => "Insert",
            Mode::Normal => "Normal",
            Mode::Visual => "Visual",
        }
    }
}

// TODO make this less awful
fn is_special(ke: &KeyEvent) -> Option<EditorAction> {
    if ke.modifiers == KeyModifiers::from_name("CONTROL").expect("Unable to check modifiers") {
        if let KeyCode::Char(c) = ke.code {
            match c {
                'c' => return Some(EditorAction::Exit),
                's' => return Some(EditorAction::Save),
                _ => return None,
            }
        }
    }

    None
}

pub fn process_key_event(ke: KeyEvent, buf: &mut EditorState) -> Result<EditorAction> {
    if let Some(action) = is_special(&ke) {
        return Ok(action);
    } else {
        match buf.mode() {
            Mode::Insert => process_insert_input(ke, buf),
            Mode::Normal => process_normal_input(ke, buf),
            Mode::Visual => process_visual_input(ke, buf),
        }
    }
}
