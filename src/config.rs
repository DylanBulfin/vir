use std::collections::HashMap;
use std::fs;

use std::io::Result;
use toml::{Table, Value};

use crate::{
    actions::{InsertAction, NormalAction, VisualAction},
    editor::TextObject,
};

struct Config {
    insert: HashMap<String, InsertAction>,
    normal: HashMap<String, NormalAction>,
    visual: HashMap<String, VisualAction>,
    textobjects: HashMap<String, TextObject>,
}

impl Config {
    pub fn init() -> Result<()> {
        let tab = Config::generate_table();

        unimplemented!()
    }

    pub fn init_insert(tab: &HashMap<String, String>) -> Result<HashMap<String, InsertAction>> {
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

    fn generate_table() -> Result<HashMap<String, String>> {
        let file = fs::read_to_string("~/.config/vir/config.toml")?
            .parse::<Table>()
            .expect("Unable to parse config.toml");

        let mut map = HashMap::new();

        for (k, v) in file {
            if let Value::String(s) = v {
                map.insert(k.to_ascii_lowercase(), s);
            }
        }

        Ok(map)
    }
}
