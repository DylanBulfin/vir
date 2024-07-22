use std::io::{stdout, Result, Write};

use crossterm::{
    cursor::{self, SetCursorStyle},
    queue,
    terminal::{self, enable_raw_mode},
};

enum CursorMode {
    BlinkLine,
    BlinkBar,
    Bar,
}

struct Cursor {
    x: usize,
    y: usize,
    mode: CursorMode,
}

pub(crate) struct Term<'a> {
    width: usize,
    height: usize,
    cursor: Cursor,
    pub text: &'a [String],
}

impl<'a> Term<'a> {
    pub fn new(text: &'a [String]) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        Ok(Term {
            width: terminal::size()?.0 as usize,
            height: terminal::size()?.1 as usize,
            cursor: Cursor {
                x: 0,
                y: 0,
                mode: CursorMode::BlinkBar,
            },
            text,
        })
    }

    pub fn redraw(&self) -> Result<()> {
        let mut stdout = stdout();

        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;

        for line in self.text.iter() {
            println!("{}", line);
            queue!(
                stdout,
                terminal::Clear(terminal::ClearType::CurrentLine),
                cursor::MoveToColumn(0)
            )?;
        }

        queue!(
            stdout,
            cursor::MoveTo(self.cursor.x as u16, self.cursor.y as u16),
            match self.cursor.mode {
                CursorMode::BlinkLine => SetCursorStyle::BlinkingBar,
                CursorMode::BlinkBar => SetCursorStyle::BlinkingBlock,
                CursorMode::Bar => SetCursorStyle::SteadyBlock,
            },
        )?;

        stdout.flush()
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;

        self.cursor.x = self.cursor.x.clamp(0, self.width - 1);
        self.cursor.y = self.cursor.y.clamp(0, self.height - 1);
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.cursor.x = x;
        self.cursor.y = y;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
