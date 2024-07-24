use std::io::Result;

use crossterm::event::{self, read};
use crossterm::terminal::disable_raw_mode;
use editor::{EditorAction, EditorState};
use modes::process_key_event;
use terminal::Term;

mod editor;
mod input;
mod terminal;
mod util;

mod modes;
mod config;

fn main_loop() -> Result<()> {
    let mut term = Term::new().unwrap();
    term.change_mode(&modes::Mode::Insert);
    let mut editor = EditorState::new(vec!["abc".to_string(), "bcdef".to_string()], term);

    editor.redraw()?;

    loop {
        let action = match read()? {
            event::Event::Key(ke) => process_key_event(ke, &mut editor)?,
            event::Event::Resize(w, h) => {
                editor.resize(w as usize, h as usize);
                EditorAction::None
            }
            _ => continue,
        };

        match action {
            EditorAction::None => (),
            EditorAction::Exit => break,
        }

        editor.redraw()?;
    }

    Ok(())
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|p| {
        disable_raw_mode().unwrap();
        print!("{}", p)
    }));
    main_loop()?;
    disable_raw_mode()
}
