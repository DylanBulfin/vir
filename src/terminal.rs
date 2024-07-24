use std::io::{stdout, Result, Write};

use crossterm::{
    cursor::{self, SetCursorStyle},
    queue,
    terminal::{self, enable_raw_mode},
};

use crate::{
    editor::{Cursor, CursorMode},
    modes::Mode,
};

pub(crate) struct Term {
    width: usize,
    height: usize,
}

impl Term {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        Ok(Term {
            width: terminal::size()?.0 as usize,
            // There was a bug i couldn't fix
            height: terminal::size()?.1 as usize, // - 1,
        })
    }

    pub fn redraw(
        &self,
        x_offset: usize,
        x: usize,
        y: usize,
        mode: &Mode,
        text: &[String],
    ) -> Result<()> {
        let mut stdout = stdout();

        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;

        for (i, line) in text.iter().enumerate() {
            let line = if line.len() <= x_offset {
                ""
            } else {
                let limit = self.width.min(line.len() - x_offset);
                &line[x_offset..x_offset + limit]
            };

            print!("{}", line);
            // Don't want to add newline on last line
            if i < text.len() - 1 {
                println!("");
                queue!(stdout, cursor::MoveToColumn(0))?;
            }
        }

        queue!(
            stdout,
            cursor::MoveTo(x as u16, y as u16),
            match mode {
                Mode::Insert => SetCursorStyle::BlinkingBar,
                Mode::Normal => SetCursorStyle::BlinkingBlock,
                Mode::Visual => SetCursorStyle::SteadyBlock,
            },
        )?;

        stdout.flush()
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
