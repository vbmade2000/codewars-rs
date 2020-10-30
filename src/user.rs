use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A structure that holds overall rank for a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct OverallRank {
    pub rank: i64,
    pub name: String,
    pub color: String,
    pub score: u64,
}

impl OverallRank {
    /// Returns a new instance of OverallRank struct.
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
    /// Returns a new instance with OverallRank struct with default values.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds overall ranks and ranks in various languages for a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct Ranks {
    pub overall: OverallRank,
    pub languages: HashMap<String, OverallRank>,
}

impl Ranks {
    /// Returns a new instance of Ranks struct.
    pub fn new() -> Self {
        Ranks {
            overall: OverallRank::new(),
            languages: HashMap::new(),
        }
    }
}

impl Default for Ranks {
    /// Returns a new instance of CodeChallenge struct with default values.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds a code challenges solved by a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeChallenges {
    pub total_authored: u64,
    pub total_completed: u64,
}

impl CodeChallenges {
    /// Returns a new instance of CodeChallenge struct.
    pub fn new() -> Self {
        CodeChallenges {
            total_authored: 0,
            total_completed: 0,
        }
    }
}

impl Default for CodeChallenges {
    /// Returns a new instance of CodeChallenge struct.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds and represent a single Codewars user.
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
    /// Returns a new instance of User struct.
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
    /// Returns a new instance of User struct with default implementation.
    fn default() -> Self {
        Self::new()
    }
}
