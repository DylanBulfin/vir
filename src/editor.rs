use std::{
    env,
    fs::File,
    io::{stdout, BufWriter, Result, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{cursor, queue, style::Stylize, terminal};

use crate::{
    actions::{InsertAction, NormalAction, VisualAction},
    config::Config,
    modes::Mode,
    term::Term,
};

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy)]
pub(crate) enum TextObject {
    Char(Position),
    Line(usize),              // Line number
    LineEnd(Position, usize), // Pos, # chars
    Word(Position, usize),    // Pos, # chars

    Selection(Position), // Position of cursor end of selection

    CancelOp,
    None,
}

impl TextObject {
    pub fn get_start(&self) -> Position {
        match self {
            TextObject::Char(p) | TextObject::LineEnd(p, _) | TextObject::Word(p, _) => *p,

            TextObject::Line(lnum) => Position::new(*lnum, 0),
            TextObject::CancelOp => panic!("CancelOp has no start position"),
            TextObject::None => panic!("None has no start position"),
            TextObject::Selection(_) => panic!("Selection has no start position"),
        }
    }

    pub fn get_end(&self) -> Position {
        match self {
            TextObject::Char(p) => Position::new(p.lnum, p.index + 1),
            TextObject::Line(lnum) => Position::new(lnum + 1, 0),
            TextObject::Word(p, c) | TextObject::LineEnd(p, c) => {
                Position::new(p.lnum, p.index + *c as usize)
            }
            TextObject::CancelOp => panic!("CancelOp has no start position"),
            TextObject::None => panic!("None has no start position"),
            TextObject::Selection(_) => panic!("Selection has no start position"),
        }
    }

    pub fn get_bounds(&self) -> (Position, Position) {
        (self.get_start(), self.get_end())
    }

    pub fn get_selection_bounds(&self, anchor: Position) -> (Position, Position) {
        if let TextObject::Selection(pos) = self {
            let largest = if pos.lnum > anchor.lnum || pos.index > anchor.index {
                pos
            } else {
                &anchor
            };
            let smallest = if largest == pos { &anchor } else { pos };

            (
                *smallest,
                Position {
                    lnum: largest.lnum,
                    index: largest.index + 1,
                },
            )
        } else {
            panic!("get_selection_bounds should only be called on Selection type")
        }
    }

    pub fn is_none(&self) -> bool {
        match &self {
            TextObject::None => true,
            _ => false,
        }
    }
}

pub struct Cursor {
    pos: Position,
}

impl Cursor {
    pub fn pos(&self) -> Position {
        self.pos
    }
}

pub(crate) struct EditorState {
    data: Vec<String>,
    term_y: usize,
    term_x: usize,
    mode: Mode,
    cursor: Cursor,
    anchor: Position, // Position of anchor in visual mode
    term: Term,
    config: Config,
}

impl EditorState {
    pub fn new(data: Vec<String>, term: Term, config: Config) -> Self {
        EditorState {
            data,
            term_y: 0,
            term_x: 0,
            mode: Mode::Insert,
            cursor: Cursor {
                pos: Position::new(0, 0),
            },
            anchor: Position::new(0, 0),
            term,
            config,
        }
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn redraw(&mut self) -> Result<()> {
        self.wrangle_cursor();
        let upper_limit = self.data.len().min(self.term_y + self.term.height());
        let (cursor, anchor) = self.get_selection_xy();
        self.term.redraw(
            self.term_x,
            cursor,
            anchor,
            &self.mode,
            &self.data[self.term_y..upper_limit],
        )
    }

    pub fn save_file(&mut self, name: &str) -> Result<()> {
        let f = File::create(name).expect("Unable to open file for writing");
        let mut writer = BufWriter::new(f);

        for line in self.data.iter() {
            writeln!(writer, "{}", line)?;
        }

        let mut stdout = stdout();

        queue!(
            stdout,
            cursor::MoveTo(0, self.term.height() as u16 / 2),
            terminal::Clear(terminal::ClearType::All),
        )?;

        print!("{}", "Successfully saved file".cyan().on_dark_grey());
        stdout.flush()?;
        sleep(Duration::from_millis(2000));

        self.redraw()?;

        Ok(())
    }

    fn wrangle_cursor(&mut self) {
        // First make sure it is within the bounds of the text buffer
        if self.cursor.pos.lnum >= self.data.len() {
            self.cursor.pos.lnum = self.data.len() - 1;
        }

        if self.cursor.pos.index >= self.data[self.cursor.pos.lnum].len() {
            // Insert mode can go one character farther right
            self.cursor.pos.index = self.data[self.cursor.pos.lnum].len()
                - if self.mode == Mode::Insert || self.data[self.cursor.pos.lnum].len() == 0 {
                    0
                } else {
                    1
                };
        }

        // Now reposition the terminal window so that it contains the cursor
        if self.cursor.pos.lnum < self.term_y {
            self.term_y = self.cursor.pos.lnum;
        } else if self.cursor.pos.lnum >= self.term_y + self.term.height() {
            self.term_y = self.cursor.pos.lnum - self.term.height() + 1;
        }

        if self.cursor.pos.index < self.term_x {
            self.term_x = self.cursor.pos.index;
        } else if self.cursor.pos.index >= self.term_x + self.term.width() {
            self.term_x = self.cursor.pos.index - self.term.width()
                + if self.mode == Mode::Insert { 1 } else { 0 };
        }
    }

    fn get_selection_xy(&self) -> ((usize, usize), (usize, usize)) {
        let cursor = (
            self.cursor.pos.index - self.term_x,
            self.cursor.pos.lnum - self.term_y,
        );
        let anchor = (
            self.anchor.index - self.term_x,
            self.anchor.lnum - self.term_y,
        );

        (cursor, anchor)
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.term.resize(width, height)
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn visual_mode(&mut self) {
        self.mode = Mode::Visual;
        self.anchor = self.cursor.pos;
    }

    pub fn get_word_textobject(&self, pos: Position) -> TextObject {
        let word_chars = match env::var("WORDCHARS") {
            Ok(wc) => wc,
            Err(_) => String::from("*?_-.[]~=&;!#$%^(){}<>"),
        };

        TextObject::Word(
            pos,
            match self.data[pos.lnum][pos.index..].split_once(|c| word_chars.contains(c)) {
                Some((w, _)) => w.len(),
                None => self.data[pos.lnum].len() - pos.index,
            },
        )
    }

    pub fn get_lineend_textobject(&self, pos: Position) -> TextObject {
        TextObject::LineEnd(pos, self.data[pos.lnum].len() - pos.index)
    }

    pub fn insert_text(&mut self, pos: Position, text: &str) {
        let (p1, p2) = self.data[pos.lnum].split_at(pos.index);
        self.data[pos.lnum] = format!("{}{}{}", p1, text, p2);
        self.cursor.pos.index += text.len()
    }

    pub fn replace(&mut self, txt_obj: TextObject, text: &str) {
        if txt_obj.is_none() {
            return;
        }

        let (start, end) = if let TextObject::Selection(_) = txt_obj {
            txt_obj.get_selection_bounds(self.anchor)
        } else {
            txt_obj.get_bounds()
        };

        self.normal_mode();

        self.data[start.lnum] = format!(
            "{}{}{}",
            &self.data[start.lnum][..start.index],
            text,
            &self.data[end.lnum][end.index..]
        );

        for lnum in start.lnum + 1..end.lnum + 1 {
            self.data.remove(lnum);
        }
    }

    pub fn insert_newline(&mut self) {
        let line = String::from(&self.data[self.cursor.pos.lnum]);
        let (p1, p2) = line.split_at(self.cursor.pos.index);

        self.data[self.cursor.pos.lnum] = p1.to_string();
        self.data.insert(self.cursor.pos.lnum + 1, p2.to_string());

        self.cursor.pos.lnum += 1;
        self.cursor.pos.index = self.term_x;
    }

    pub fn delete_newline(&mut self, lnum: usize) {
        let end_index = self.data[lnum].len();

        let line = self.data.remove(lnum + 1);
        self.data[lnum] = format!("{}{}", self.data[lnum], line);

        self.cursor.pos.index = end_index;
    }

    pub fn delete(&mut self, txt_obj: TextObject) {
        if txt_obj.is_none() {
            return;
        }

        let (start, end) = if let TextObject::Selection(_) = txt_obj {
            txt_obj.get_selection_bounds(self.anchor)
        } else {
            txt_obj.get_bounds()
        };

        self.normal_mode();

        self.data[start.lnum] = format!(
            "{}{}",
            &self.data[start.lnum][..start.index],
            &self.data[end.lnum][end.index..]
        );

        for lnum in start.lnum + 1..end.lnum + 1 {
            self.data.remove(lnum);
        }
    }

    pub fn insert_bind(&self, key: &str) -> Option<InsertAction> {
        self.config.insert(key)
    }

    pub fn normal_bind(&self, key: &str) -> Option<NormalAction> {
        self.config.normal(key)
    }

    pub fn visual_bind(&self, key: &str) -> Option<VisualAction> {
        self.config.visual(key)
    }

    pub fn textobject_bind(&self, key: &str) -> Option<TextObject> {
        self.config.textobject(key)
    }

    pub fn backspace(&mut self, pos: Position) {
        self.data[pos.lnum].remove(pos.index - 1);
        self.cursor_left()
    }

    pub fn indent(&mut self) {
        // Want to make this configurable
        let spaces = &(0..self.config.option("tabstop"))
            .map(|_| ' ')
            .collect::<String>();
        let (p1, p2) = self.data[self.cursor.pos.lnum].split_at(self.cursor.pos.index);
        self.data[self.cursor.pos.lnum] = format!("{}{}{}", p1, spaces, p2);

        self.cursor.pos.index += self.config.option("tabstop") as usize;
    }

    pub fn cursor_right(&mut self) {
        self.cursor.pos.index += 1;
    }

    pub fn cursor_left(&mut self) {
        self.cursor.pos.index = self.cursor.pos.index.saturating_sub(1);
    }

    pub fn cursor_down(&mut self) {
        self.cursor.pos.lnum += 1;
    }

    pub fn cursor_up(&mut self) {
        self.cursor.pos.lnum = self.cursor.pos.lnum.saturating_sub(1);
    }

    pub fn line_start(&mut self) {
        self.cursor.pos.index = 0;
    }

    pub fn line_end(&mut self) {
        self.cursor.pos.index = self.data[self.cursor.pos.lnum].len();
    }
}
