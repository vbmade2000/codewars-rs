use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeChallenges {
    pub total_authored: u64,
    pub total_completed: u64,
}

impl CodeChallenges {
    pub fn new() -> Self {
        CodeChallenges {
            total_authored: 0,
            total_completed: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub username: String,
    pub honor: Option<i32>,
    pub clan: Option<String>,
    pub skills: Vec<String>,
    pub code_challenges: CodeChallenges,
}

impl User {
    pub fn new() -> Self {
        User {
            name: "".to_string(),
            username: "".to_string(),
            honor: None,
            clan: None,
            skills: vec![],
            code_challenges: CodeChallenges::new(),
        }
    }
}
