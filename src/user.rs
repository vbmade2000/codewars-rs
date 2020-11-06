use serde::{Deserialize, Serialize};
use serde_json::Value;
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

    /// Extrats and fill up User fields from JSON Values
    pub fn from_json(&mut self, response_json: &Value) {
        // Extract single fields
        self.name = response_json
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.username = response_json
            .get("username")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.clan = Some(
            response_json
                .get("clan")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        );
        self.honor = response_json.get("honor").unwrap().as_i64();
        self.leaderboard_position = response_json.get("leaderboardPosition").unwrap().as_i64();

        // Extract Vectors
        let user_skills = response_json.get("skills").unwrap().as_array();
        if user_skills.is_some() {
            for skill in user_skills.unwrap() {
                self.skills.push(String::from(skill.as_str().unwrap()));
            }
        }

        // Fill up CodeChallenge structure
        let user_codechallanges = response_json
            .get("codeChallenges")
            .unwrap()
            .as_object()
            .unwrap();
        self.code_challenges.total_authored = user_codechallanges
            .get("totalAuthored")
            .unwrap()
            .as_u64()
            .unwrap();
        self.code_challenges.total_completed = user_codechallanges
            .get("totalCompleted")
            .unwrap()
            .as_u64()
            .unwrap();

        /* Fill up Ranks structure */
        let user_ranks = response_json.get("ranks").unwrap().as_object().unwrap();
        //--  Fill Rank->Overall structure
        let overall_rank = user_ranks.get("overall").unwrap().as_object().unwrap();
        self.ranks.overall.rank = overall_rank.get("rank").unwrap().as_i64().unwrap();
        self.ranks.overall.name = overall_rank
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.ranks.overall.color =
            String::from(overall_rank.get("color").unwrap().as_str().unwrap());
        self.ranks.overall.score = overall_rank.get("score").unwrap().as_u64().unwrap();
        //-- Fill up Rank->Languages Map
        let user_languages = user_ranks.get("languages").unwrap().as_object().unwrap();
        for lang in user_languages {
            let rank = lang.1.get("rank").unwrap().as_i64().unwrap();
            let name = String::from(lang.1.get("name").unwrap().as_str().unwrap());
            let color = String::from(lang.1.get("color").unwrap().as_str().unwrap());
            let score = lang.1.get("score").unwrap().as_u64().unwrap();
            let mut temp_overall_rank = OverallRank::new();
            temp_overall_rank.rank = rank;
            temp_overall_rank.name = name;
            temp_overall_rank.color = color;
            temp_overall_rank.score = score;

            self.ranks
                .languages
                .insert(lang.0.clone(), temp_overall_rank);
        }
    }
}

impl Default for User {
    /// Returns a new instance of User struct with default implementation.
    fn default() -> Self {
        Self::new()
    }
}
