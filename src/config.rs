use std::fs;
use std::{collections::HashMap, hash::Hash};

use std::io::Result;
use toml::{Table, Value};

use crate::{
    actions::{InsertAction, NormalAction, VisualAction},
    editor::{Position, TextObject},
};

pub struct Config {
    insert: HashMap<String, InsertAction>,
    normal: HashMap<String, NormalAction>,
    visual: HashMap<String, VisualAction>,
    textobjects: HashMap<String, TextObject>,
    options: HashMap<String, i64>,
}

impl Config {
    pub fn init() -> Result<Config> {
        let tab = match Config::generate_table() {
            Ok(t) => t,
            Err(_) => HashMap::new(),
        };

        Ok(Config {
            insert: Config::init_insert(&tab).unwrap(),
            normal: Config::init_normal(&tab).unwrap(),
            visual: Config::init_visual(&tab).unwrap(),
            textobjects: Config::init_textobjects(&tab).unwrap(),
            options: Config::init_options(&tab).unwrap(),
        })
    }

    pub fn insert(&self, key: &str) -> Option<InsertAction> {
        self.insert.get(key).map(|b| *b)
    }

    pub fn normal(&self, key: &str) -> Option<NormalAction> {
        self.normal.get(key).map(|b| *b)
    }

    pub fn visual(&self, key: &str) -> Option<VisualAction> {
        self.visual.get(key).map(|b| *b)
    }

    pub fn textobject(&self, key: &str) -> Option<TextObject> {
        self.textobjects.get(key).map(|b| *b)
    }

    pub fn option(&self, key: &str) -> i64 {
        self.options[key]
    }

    fn init_insert(tab: &HashMap<String, Value>) -> Result<HashMap<String, InsertAction>> {
        let mut map = HashMap::new();

        map.insert(
            if tab.contains_key("delforw") {
                tab.get("delforw").unwrap().to_string()
            } else {
                "delete".to_string()
            },
            InsertAction::DelForw,
        );

        map.insert(
            if tab.contains_key("delback") {
                tab.get("delback").unwrap().to_string()
            } else {
                "backspace".to_string()
            },
            InsertAction::DelBack,
        );

        map.insert(
            if tab.contains_key("newline") {
                tab.get("newline").unwrap().to_string()
            } else {
                "enter".to_string()
            },
            InsertAction::NewLine,
        );

        map.insert(
            if tab.contains_key("indent") {
                tab.get("indent").unwrap().to_string()
            } else {
                "tab".to_string()
            },
            InsertAction::Indent,
        );

        map.insert(
            if tab.contains_key("normalmode") {
                tab.get("indent").unwrap().to_string()
            } else {
                "esc".to_string()
            },
            InsertAction::NormalMode,
        );

        map.insert(
            if tab.contains_key("up") {
                tab.get("up").unwrap().to_string()
            } else {
                "up".to_string()
            },
            InsertAction::Up,
        );

        map.insert(
            if tab.contains_key("down") {
                tab.get("down").unwrap().to_string()
            } else {
                "down".to_string()
            },
            InsertAction::Down,
        );

        map.insert(
            if tab.contains_key("left") {
                tab.get("left").unwrap().to_string()
            } else {
                "left".to_string()
            },
            InsertAction::Left,
        );

        map.insert(
            if tab.contains_key("right") {
                tab.get("right").unwrap().to_string()
            } else {
                "right".to_string()
            },
            InsertAction::Right,
        );

        Ok(map)
    }

    fn init_normal(tab: &HashMap<String, Value>) -> Result<HashMap<String, NormalAction>> {
        let mut map = HashMap::new();

        map.insert(
            if tab.contains_key("replacechar") {
                tab.get("replacechar").unwrap().to_string()
            } else {
                "r".to_string()
            },
            NormalAction::ReplaceChar,
        );

        map.insert(
            if tab.contains_key("delete") {
                tab.get("delete").unwrap().to_string()
            } else {
                "d".to_string()
            },
            NormalAction::Delete,
        );

        map.insert(
            if tab.contains_key("deletechar") {
                tab.get("deletechar").unwrap().to_string()
            } else {
                "x".to_string()
            },
            NormalAction::DeleteChar,
        );

        map.insert(
            if tab.contains_key("change") {
                tab.get("change").unwrap().to_string()
            } else {
                "c".to_string()
            },
            NormalAction::Change,
        );

        map.insert(
            if tab.contains_key("yank") {
                tab.get("yank").unwrap().to_string()
            } else {
                "y".to_string()
            },
            NormalAction::Yank,
        );

        map.insert(
            if tab.contains_key("linestart") {
                tab.get("linestart").unwrap().to_string()
            } else {
                "^".to_string()
            },
            NormalAction::LineStart,
        );

        map.insert(
            if tab.contains_key("lineend") {
                tab.get("lineend").unwrap().to_string()
            } else {
                "$".to_string()
            },
            NormalAction::LineEnd,
        );

        map.insert(
            if tab.contains_key("searchmode") {
                tab.get("searchmode").unwrap().to_string()
            } else {
                "/".to_string()
            },
            NormalAction::SearchMode,
        );

        map.insert(
            if tab.contains_key("insertmode") {
                tab.get("insertmode").unwrap().to_string()
            } else {
                "u".to_string()
            },
            NormalAction::InsertMode,
        );

        map.insert(
            if tab.contains_key("visualmode") {
                tab.get("visualmode").unwrap().to_string()
            } else {
                "v".to_string()
            },
            NormalAction::VisualMode,
        );

        map.insert(
            if tab.contains_key("exit") {
                tab.get("exit").unwrap().to_string()
            } else {
                "q".to_string()
            },
            NormalAction::Exit,
        );

        map.insert(
            if tab.contains_key("up") {
                tab.get("up").unwrap().to_string()
            } else {
                "e".to_string()
            },
            NormalAction::Up,
        );

        map.insert(
            if tab.contains_key("down") {
                tab.get("down").unwrap().to_string()
            } else {
                "n".to_string()
            },
            NormalAction::Down,
        );

        map.insert(
            if tab.contains_key("left") {
                tab.get("left").unwrap().to_string()
            } else {
                "m".to_string()
            },
            NormalAction::Left,
        );

        map.insert(
            if tab.contains_key("right") {
                tab.get("right").unwrap().to_string()
            } else {
                "i".to_string()
            },
            NormalAction::Right,
        );

        Ok(map)
    }

    fn init_visual(tab: &HashMap<String, Value>) -> Result<HashMap<String, VisualAction>> {
        let mut map = HashMap::new();

        map.insert(
            if tab.contains_key("replacechar") {
                tab.get("replacechar").unwrap().to_string()
            } else {
                "r".to_string()
            },
            VisualAction::ReplaceChar,
        );

        map.insert(
            if tab.contains_key("delete") {
                tab.get("delete").unwrap().to_string()
            } else {
                "d".to_string()
            },
            VisualAction::Delete,
        );

        map.insert(
            if tab.contains_key("deletechar") {
                tab.get("deletechar").unwrap().to_string()
            } else {
                "x".to_string()
            },
            VisualAction::DeleteChar,
        );

        map.insert(
            if tab.contains_key("change") {
                tab.get("change").unwrap().to_string()
            } else {
                "c".to_string()
            },
            VisualAction::Change,
        );

        map.insert(
            if tab.contains_key("yank") {
                tab.get("yank").unwrap().to_string()
            } else {
                "y".to_string()
            },
            VisualAction::Yank,
        );

        map.insert(
            if tab.contains_key("linestart") {
                tab.get("linestart").unwrap().to_string()
            } else {
                "^".to_string()
            },
            VisualAction::LineStart,
        );

        map.insert(
            if tab.contains_key("lineend") {
                tab.get("lineend").unwrap().to_string()
            } else {
                "$".to_string()
            },
            VisualAction::LineEnd,
        );

        map.insert(
            if tab.contains_key("up") {
                tab.get("up").unwrap().to_string()
            } else {
                "e".to_string()
            },
            VisualAction::Up,
        );

        map.insert(
            if tab.contains_key("down") {
                tab.get("down").unwrap().to_string()
            } else {
                "n".to_string()
            },
            VisualAction::Down,
        );

        map.insert(
            if tab.contains_key("left") {
                tab.get("left").unwrap().to_string()
            } else {
                "m".to_string()
            },
            VisualAction::Left,
        );

        map.insert(
            if tab.contains_key("right") {
                tab.get("right").unwrap().to_string()
            } else {
                "i".to_string()
            },
            VisualAction::Right,
        );

        map.insert(
            if tab.contains_key("normalmode") {
                tab.get("normalmode").unwrap().to_string()
            } else {
                "esc".to_string()
            },
            VisualAction::NormalMode,
        );
        
        map.insert(
            if tab.contains_key("exit") {
                tab.get("exit").unwrap().to_string()
            } else {
                "q".to_string()
            },
            VisualAction::Exit,
        );

        Ok(map)
    }

    fn init_textobjects(tab: &HashMap<String, Value>) -> Result<HashMap<String, TextObject>> {
        let mut map = HashMap::new();

        map.insert(
            if tab.contains_key("char") {
                tab.get("char").unwrap().to_string()
            } else {
                "i".to_string()
            },
            TextObject::Char(Position::new(0, 0)),
        );

        map.insert(
            if tab.contains_key("line") {
                tab.get("line").unwrap().to_string()
            } else {
                "V".to_string()
            },
            TextObject::Line(0),
        );

        map.insert(
            if tab.contains_key("word") {
                tab.get("word").unwrap().to_string()
            } else {
                "w".to_string()
            },
            TextObject::Word(Position::new(0, 0), 0),
        );

        map.insert(
            if tab.contains_key("lineend") {
                tab.get("lineend").unwrap().to_string()
            } else {
                "$".to_string()
            },
            TextObject::LineEnd(Position::new(0, 0), 0),
        );

        map.insert(
            if tab.contains_key("cancelop") {
                tab.get("exit").unwrap().to_string()
            } else {
                "esc".to_string()
            },
            TextObject::CancelOp,
        );

        Ok(map)
    }

    fn init_options(tab: &HashMap<String, Value>) -> Result<HashMap<String, i64>> {
        let mut map = HashMap::new();

        map.insert(
            "tabstop".to_string(),
            if tab.contains_key("tabstop") && tab["tabstop"].is_integer() {
                tab["tabstop"].as_integer().unwrap()
            } else {
                4
            },
        );

        Ok(map)
    }

    fn generate_table() -> Result<HashMap<String, Value>> {
        let file = fs::read_to_string("~/.config/vir/config.toml")?
            .parse::<Table>()
            .expect("Unable to parse config.toml");

        let mut map = HashMap::new();

        for (k, v) in file {
            map.insert(k.to_ascii_lowercase(), v);
        }

        Ok(map)
    }
}
