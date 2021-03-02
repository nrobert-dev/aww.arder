use serde::{Deserialize, Serialize};

pub const KEY: &'static str = "yew.aww.ards.database";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub members : Vec<String>
}

impl Database {
    pub fn new() -> Self {
        Database { members : Vec::new()}
    }
}
