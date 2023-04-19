pub enum Status {
    Open,
    InProgress,
    Resolved,
    Close,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Story>,
}

impl Epic {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Epic {
            name: name.into(),
            description: description.into(),
            status: Status::Open,
            stories: vec![],
        }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Story {
            name: name.into(),
            description: description.into(),
            status: Status::Open,
        }
    }
}

pub struct DBState {
    pub last_item_id: u64,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
}
