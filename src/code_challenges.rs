pub struct Rank {
    pub id: u64,
    pub name: String,
    pub color: String,
}

// Also used to store ApprovedBy details
pub struct CreatedBy {
    pub username: String,
    pub url: String,
}

pub struct Unresolved {
    pub issues: u64,
    pub suggestions: u64,
}

pub struct CodeChallenge {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub category: String,
    pub published_at: String,
    pub approved_at: String,
    pub languages: Vec<String>,
    pub url: String,
    pub rank: Rank,
    pub created_at: String,
    pub created_by: CreatedBy,
    pub approved_by: CreatedBy,
    pub description: String,
    pub total_attempts: u64,
    pub total_completed: u64,
    pub total_stars: u64,
    pub vote_scores: u64,
    pub tags: Vec<String>,
    pub contributors_wanted: bool,
    pub unresolved: Unresolved,
}
