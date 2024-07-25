use std::env::args;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Result, Write};

use actions::EditorAction;
use config::Config;
use crossterm::event::{self, read};
use crossterm::terminal::{self, disable_raw_mode};
use crossterm::{cursor, queue};
use editor::EditorState;
use modes::process_key_event;
use term::Term;

mod actions;
mod editor;
mod term;

mod config;
mod modes;

fn main_loop(filename: &str, data: Vec<String>) -> Result<()> {
    let term = Term::new()?;
    let config = Config::init()?;
    let mut editor = EditorState::new(data, term, config);

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
            EditorAction::Save => editor.save_file(filename)?,
        }

        editor.redraw()?;
    }

    queue!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;

    stdout().flush()?;

    Ok(())
}

fn read_file(name: &str) -> Result<Vec<String>> {
    let f = File::open(name).expect("Couldn't open file for reading");
    let buf = BufReader::new(f);

    buf.lines().collect()
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|p| {
        disable_raw_mode().unwrap_or_default();
        println!("{}", p)
    }));

    let filename = &args().collect::<Vec<_>>()[1];
    let data = read_file(filename)?;

    let _a = main_loop(filename, data);

    disable_raw_mode()
}
