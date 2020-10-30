/// A structure that holds a rank for code challenge.
pub struct Rank {
    pub id: i64,
    pub name: String,
    pub color: String,
}

impl Rank {
    /// Returns a new instance of Rank struct
    pub fn new() -> Self {
        Rank {
            id: 0,
            name: "".to_string(),
            color: "".to_string(),
        }
    }
}

impl Default for Rank {
    /// Returns a new instance of Rank struct with default values.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds a short details of a user that created code challenge.
pub struct CreatedBy {
    pub username: String,
    pub url: String,
}

impl CreatedBy {
    /// Returns a new instance of CreatedBy struct
    pub fn new() -> Self {
        CreatedBy {
            username: "".to_string(),
            url: "".to_string(),
        }
    }
}

impl Default for CreatedBy {
    /// Returns a new instance of CreatedBy struct with default values.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds unresolved issues
pub struct Unresolved {
    pub issues: u64,
    pub suggestions: u64,
}

impl Unresolved {
    /// Returns a new instance of Unresolved struct.
    pub fn new() -> Self {
        Unresolved {
            issues: 0,
            suggestions: 0,
        }
    }
}

impl Default for Unresolved {
    /// Returns a new instance of Unresolved struct with default values.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds a single Codewars code challenge.
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
    pub vote_score: u64,
    pub tags: Vec<String>,
    pub contributors_wanted: bool,
    pub unresolved: Unresolved,
}

impl CodeChallenge {
    /// Returns a new instance of CodeChallenge struct.
    pub fn new() -> Self {
        CodeChallenge {
            id: "".to_string(),
            name: "".to_string(),
            slug: "".to_string(),
            category: "".to_string(),
            published_at: "".to_string(),
            approved_at: "".to_string(),
            languages: vec![],
            url: "".to_string(),
            rank: Rank::new(),
            created_at: "".to_string(),
            created_by: CreatedBy::new(),
            approved_by: CreatedBy::new(),
            description: "".to_string(),
            total_attempts: 0,
            total_completed: 0,
            total_stars: 0,
            vote_score: 0,
            tags: vec![],
            contributors_wanted: false,
            unresolved: Unresolved::new(),
        }
    }
}

impl Default for CodeChallenge {
    /// Returns a new instance of CodeChallenge struct with default values.
    fn default() -> Self {
        Self::new()
    }
}
