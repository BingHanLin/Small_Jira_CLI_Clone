use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Done,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u16>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name: name,
            description: description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name: name,
            description: description,
            status: Status::Open,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DBState {
    pub last_item_id: u16,
    pub epics: HashMap<u16, Epic>,
    pub stories: HashMap<u16, Story>,
}
