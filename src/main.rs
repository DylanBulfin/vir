use std::io::Result;

use crossterm::terminal::disable_raw_mode;
use editor::Editor;
use terminal::Term;

mod editor;
mod input;
mod terminal;
mod util;

fn main() -> Result<()> {
    let editor = Editor::new(vec!["abc".to_string(), "bcd".to_string()]);
    let term = Term::new(editor.get_slice(2)).unwrap();
    term.redraw()?;

    disable_raw_mode()?;
    loop {}
}
