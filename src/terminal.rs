use std::io::{stdout, Result, Write};

use crossterm::{
    cursor::{self, SetCursorStyle},
    queue,
    terminal::{self, enable_raw_mode},
};

use crate::modes::Mode;

enum CursorMode {
    BlinkLine,
    BlinkBar,
    Bar,
}

pub struct Cursor {
    x: usize,
    y: usize,
    mode: CursorMode,
}

impl Cursor {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

pub(crate) struct Term {
    width: usize,
    height: usize,
    cursor: Cursor,
    x_offset: usize,
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
            height: terminal::size()?.1 as usize,
            cursor: Cursor {
                x: 0,
                y: 0,
                mode: CursorMode::BlinkBar,
            },
            x_offset: 0,
        })
    }

    pub fn redraw(&self, text: &[String]) -> Result<()> {
        let mut stdout = stdout();

        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;

        for line in text.iter() {
            let line = if line.len() <= self.x_offset {
                ""
            } else {
                let limit = self.width.min(line.len() - self.x_offset);
                &line[self.x_offset..self.x_offset + limit]
            };
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

    pub fn change_mode(&mut self, mode: &Mode) {
        match mode {
            Mode::Insert => self.cursor.mode = CursorMode::BlinkLine,
            Mode::Normal => self.cursor.mode = CursorMode::BlinkBar,
            Mode::Visual => self.cursor.mode = CursorMode::Bar,
        }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.cursor.x = x;
        self.cursor.y = y;
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x < self.width - 1 {
            self.cursor.x += 1;
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor.y < self.height - 1 {
            self.cursor.y += 1;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cursor_x(&self) -> usize {
        self.cursor.x
    }

    pub fn cursor_y(&self) -> usize {
        self.cursor.y
    }
}
