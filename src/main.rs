use std::io::Result;

use crossterm::terminal::disable_raw_mode;
use editor::Buffer;
use terminal::Term;

mod editor;
mod input;
mod terminal;
mod util;

mod modes;

fn main() -> Result<()> {
    let editor = Buffer::new(vec!["abc".to_string(), "bcd".to_string()]);
    let term = Term::new(editor.get_slice(2)).unwrap();
    term.redraw()?;

    disable_raw_mode()?;
    loop {}
}
