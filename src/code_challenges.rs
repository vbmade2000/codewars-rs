use serde_json::Value;

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

    pub fn from_json(&mut self, response_json: &Value) {
        let rank_json = response_json.get("rank").unwrap().as_object().unwrap();
        let rank_id = rank_json.get("id").unwrap().as_i64().unwrap();
        let rank_name = rank_json.get("name").unwrap().as_str().unwrap().to_string();
        let rank_color = rank_json
            .get("color")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.id = rank_id;
        self.name = rank_name;
        self.color = rank_color;
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

    pub fn from_json(&mut self, response_json: &Value) {
        let created_by_json = response_json.get("createdBy").unwrap().as_object().unwrap();
        let username = created_by_json
            .get("username")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let url = created_by_json
            .get("url")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.username = username;
        self.url = url;
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

    pub fn from_json(&mut self, response_json: &Value) {
        let unresolved_json = response_json
            .get("unresolved")
            .unwrap()
            .as_object()
            .unwrap();
        let unresolved_issues = unresolved_json.get("issues").unwrap().as_u64().unwrap();
        let unresolved_suggestions = unresolved_json
            .get("suggestions")
            .unwrap()
            .as_u64()
            .unwrap();
        self.issues = unresolved_issues;
        self.suggestions = unresolved_suggestions;
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

    pub fn from_json(&mut self, response_json: &Value) {
        // Fill up single values
        let id = response_json
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.id = id;
        let name = response_json
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.name = name;
        let slug = response_json
            .get("slug")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.slug = slug;
        let category = response_json
            .get("category")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.category = category;
        let published_at = response_json
            .get("publishedAt")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.published_at = published_at;
        let approved_at = response_json
            .get("approvedAt")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.approved_at = approved_at;

        let url = response_json
            .get("url")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.url = url;

        let created_at = response_json
            .get("createdAt")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.created_at = created_at;

        let description = response_json
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.description = description;

        let total_attempts = response_json
            .get("totalAttempts")
            .unwrap()
            .as_u64()
            .unwrap();
        self.total_attempts = total_attempts;

        let total_completed = response_json
            .get("totalCompleted")
            .unwrap()
            .as_u64()
            .unwrap();
        self.total_completed = total_completed;

        let total_stars = response_json.get("totalStars").unwrap().as_u64().unwrap();
        self.total_stars = total_stars;

        let vote_score = response_json.get("voteScore").unwrap().as_u64().unwrap();
        self.vote_score = vote_score;

        let contributors_wanted = response_json
            .get("contributorsWanted")
            .unwrap()
            .as_bool()
            .unwrap();
        self.contributors_wanted = contributors_wanted;

        let mut unresolved = Unresolved::new();
        unresolved.from_json(response_json);
        self.unresolved = unresolved;

        let mut rank = Rank::new();
        rank.from_json(&response_json);
        self.rank = rank;

        let mut created_by = CreatedBy::new();
        created_by.from_json(response_json);
        self.created_by = created_by;

        let mut approved_by = CreatedBy::new();
        approved_by.from_json(response_json);
        self.approved_by = approved_by;

        let languages_json = response_json.get("languages").unwrap().as_array().unwrap();
        let mut languages: Vec<String> = Vec::new();
        for lang in languages_json {
            languages.push(lang.as_str().unwrap().to_string());
        }
        self.languages = languages;

        let tags_json = response_json.get("tags").unwrap().as_array().unwrap();
        let mut tags: Vec<String> = Vec::new();
        for tg in tags_json {
            tags.push(tg.as_str().unwrap().to_string());
        }
        self.tags = tags;
    }
}

impl Default for CodeChallenge {
    /// Returns a new instance of CodeChallenge struct with default values.
    fn default() -> Self {
        Self::new()
    }
}
