pub enum Status {
    Open,
    InProgress,
    Done,
}

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

pub struct DBState {
    pub last_item_id: u16,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
}
