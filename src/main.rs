use std::fs::File;
use std::io::{stdout, Result, Write};

use actions::EditorAction;
use config::Config;
use crossterm::event::{self, read};
use crossterm::terminal::disable_raw_mode;
use crossterm::{cursor, queue};
use editor::{Cursor, EditorState};
use modes::process_key_event;
use terminal::Term;

mod actions;
mod editor;
mod terminal;
mod util;

mod config;
mod modes;

fn main_loop() -> Result<()> {
    let term = Term::new()?;
    let config = Config::init()?;
    let mut editor = EditorState::new(vec!["abc".to_string(), "bcdef".to_string()], term, config);

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
        let mut file = File::create("foo.txt").unwrap();
        file.write_all(p.to_string().as_bytes()).unwrap();
        disable_raw_mode().unwrap();
        print!("{}", p)
    }));
    main_loop();

    disable_raw_mode()
}
