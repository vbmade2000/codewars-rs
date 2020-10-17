use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub username: String,
    pub honor: Option<i32>,
    pub clan: Option<String>,
}

impl User {
    pub fn new() -> Self {
        User {
            name: "".to_string(),
            username: "".to_string(),
            honor: None,
            clan: None,
        }
    }
}
