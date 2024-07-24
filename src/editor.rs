use std::{env, io::Result};

use crate::{modes::Mode, terminal::Term};

#[derive(Clone, Copy)]
pub(crate) struct Position {
    lnum: usize,
    index: usize,
}

impl Position {
    pub fn new(lnum: usize, index: usize) -> Self {
        Self { lnum, index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn lnum(&self) -> usize {
        self.lnum
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            lnum: value.0,
            index: value.1,
        }
    }
}

pub(crate) enum TextObject {
    Char(Position),        // Line number, index
    Line(usize),           // Line number
    LineEnd(Position),     // Line number, start_index
    Lines(usize, u16),     // Line number, count
    Word(Position, usize), // Line number, length

    // (start_lnum, start_index), (end_lnum, end_index)
    Other(Position, Position),
}

impl TextObject {
    pub fn get_start(&self) -> Position {
        match self {
            TextObject::Char(p)
            | TextObject::LineEnd(p)
            | TextObject::Word(p, _)
            | TextObject::Other(p, _) => *p,

            TextObject::Line(lnum) | TextObject::Lines(lnum, _) => Position::new(*lnum, 0),
        }
    }

    pub fn get_end(&self) -> Position {
        match self {
            TextObject::Char(p) => Position::new(p.lnum, p.index + 1),
            TextObject::Line(lnum) => Position::new(lnum + 1, 0),
            TextObject::LineEnd(p) => Position::new(p.lnum + 1, 0),
            TextObject::Lines(lnum, c) => Position::new(*lnum + *c as usize, 0),
            TextObject::Word(p, c) => Position::new(p.lnum, p.index + *c as usize),
            TextObject::Other(_, p) => *p,
        }
    }

    pub fn get_bounds(&self) -> (Position, Position) {
        (self.get_start(), self.get_end())
    }
}

pub fn get_word_textobject(pos: Position, line: &str) -> TextObject {
    let word_chars = match env::var("WORDCHARS") {
        Ok(wc) => wc,
        Err(_) => String::from("*?_-.[]~=&;!#$%^(){}<>"),
    };

    TextObject::Word(
        pos,
        match line[pos.index..].split_once(|c| word_chars.contains(c)) {
            Some((w, _)) => w.len(),
            None => line.len() - pos.index,
        },
    )
}

pub(crate) struct EditorState {
    data: Vec<String>,
    term_y: usize,
    mode: Mode,
    term: Term,
}

impl EditorState {
    pub fn new(data: Vec<String>, term: Term) -> Self {
        EditorState {
            data,
            term_y: 0,
            mode: Mode::Insert,
            term,
        }
    }

    pub fn redraw(&mut self) -> Result<()> {
        let upper_limit = self.data.len().min(self.term_y + self.term.height());
        self.term.redraw(
            self.mode == Mode::Insert,
            &self.data[self.term_y..upper_limit],
        )
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.term.resize(width, height)
    }

    pub fn cursor_pos(&self) -> Position {
        Position {
            lnum: self.term_y + self.term.cursor_y(),
            index: self.term.cursor_x(),
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
    
    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.term.change_mode(&self.mode)
    }
    
    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.term.change_mode(&self.mode)
    }

    pub fn visual_mode(&mut self) {
        self.mode = Mode::Visual;
        self.term.change_mode(&self.mode)
    }

    pub fn insert(&mut self, pos: Position, text: &str) {
        let (p1, p2) = self.data[pos.lnum].split_at(pos.index);
        self.data[pos.lnum] = format!("{}{}{}", p1, text, p2);
        self.term
            .move_to(self.term.cursor_x() + text.len(), self.term.cursor_y());
    }

    pub fn insert_newline(&mut self, pos: Position) {
        let line = String::from(&self.data[pos.lnum]);
        let (p1, p2) = line.split_at(pos.index);

        self.data[pos.lnum] = p1.to_string();
        self.data.insert(pos.lnum + 1, p2.to_string());

        self.term.move_to(0, self.term.cursor_y() + 1)
    }

    pub fn delete_newline(&mut self, lnum: usize) {
        let end_index = self.data[lnum].len();

        let line = self.data.remove(lnum + 1);
        self.data[lnum] = format!("{}{}", self.data[lnum], line);

        self.term.move_to(end_index, lnum);
    }

    pub fn delete(&mut self, txt_obj: TextObject) {
        let (start, end) = txt_obj.get_bounds();

        self.data[start.lnum] = format!(
            "{}{}",
            &self.data[start.lnum][..start.index],
            &self.data[end.lnum][end.index..]
        );

        for lnum in start.lnum + 1..end.lnum + 1 {
            self.data.remove(lnum);
        }
    }

    pub fn backspace(&mut self, pos: Position) {
        self.data[pos.lnum].remove(pos.index - 1);
        self.cursor_left()
    }

    pub fn cursor_right(&mut self) {
        self.term.cursor_right()
    }

    pub fn cursor_left(&mut self) {
        self.term.cursor_left()
    }

    pub fn cursor_down(&mut self) {
        self.term.cursor_down()
    }

    pub fn cursor_up(&mut self) {
        self.term.cursor_up()
    }
}
