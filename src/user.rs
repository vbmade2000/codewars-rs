use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct OverallRank {
    pub rank: i64,
    pub name: String,
    pub color: String,
    pub score: u64,
}

impl OverallRank {
    pub fn new() -> Self {
        OverallRank {
            rank: 0,
            name: "".to_string(),
            color: "".to_string(),
            score: 0,
        }
    }
}

impl Default for OverallRank {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ranks {
    pub overall: OverallRank,
    pub languages: HashMap<String, OverallRank>,
}

impl Ranks {
    pub fn new() -> Self {
        Ranks {
            overall: OverallRank::new(),
            languages: HashMap::new(),
        }
    }
}

impl Default for Ranks {
    fn default() -> Self {
        Self::new()
    }
}

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

impl Default for CodeChallenges {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub username: String,
    pub honor: Option<i64>,
    pub clan: Option<String>,
    pub skills: Vec<String>,
    pub code_challenges: CodeChallenges,
    pub ranks: Ranks,
    pub leaderboard_position: Option<i64>,
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
            ranks: Ranks::new(),
            leaderboard_position: None,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}
