pub struct CompletedChallenge {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub completed_languages: Vec<String>,
    pub completed_at: String,
}

impl CompletedChallenge {
    pub fn new() -> Self {
        CompletedChallenge {
            id: "".to_string(),
            name: "".to_string(),
            slug: "".to_string(),
            completed_languages: vec![],
            completed_at: "".to_string(),
        }
    }
}

pub struct AuthoredChallenge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rank: i64,
    pub rank_name: String,
    pub tags: Vec<String>,
    pub languages: Vec<String>,
}

impl AuthoredChallenge {
    pub fn new() -> Self {
        AuthoredChallenge {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            rank: 0,
            rank_name: "".to_string(),
            tags: vec![],
            languages: vec![],
        }
    }
}
