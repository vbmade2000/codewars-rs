use serde_json::Value;

/// A structure that holds a single completed challenge.
pub struct CompletedChallenge {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub completed_languages: Vec<String>,
    pub completed_at: String,
}

impl CompletedChallenge {
    /// Returns a new instance of CompletedChallenge struct.
    pub fn new() -> Self {
        CompletedChallenge {
            id: "".to_string(),
            name: "".to_string(),
            slug: "".to_string(),
            completed_languages: vec![],
            completed_at: "".to_string(),
        }
    }

    /// Extracts fields for a struct from supplied JSON and fill up struct fields with those values
    pub fn from_json(&mut self, response_json: &Value) {
        // Extract single fields
        let challenge_id = response_json.get("id").unwrap().as_str().unwrap();
        let challenge_name = response_json.get("name").unwrap().as_str().unwrap();
        let challenge_slug = response_json.get("slug").unwrap().as_str().unwrap();
        let challenge_completed_at = response_json.get("completedAt").unwrap().as_str().unwrap();

        // Extract Vectors
        let completed_languages = response_json
            .get("completedLanguages")
            .unwrap()
            .as_array()
            .unwrap();
        for completed_language in completed_languages {
            self.completed_languages
                .push(String::from(completed_language.as_str().unwrap()));
        }

        // Fill up the structure
        self.id = challenge_id.to_string();
        self.name = challenge_name.to_string();
        self.slug = challenge_slug.to_string();
        self.completed_at = challenge_completed_at.to_string();
        // self.completed_languages.push(completed_languages);
    }
}

impl Default for CompletedChallenge {
    /// Returns a new instance of CompletedChallenge struct with default values.
    fn default() -> Self {
        Self::new()
    }
}

/// A structure that holds a single authored challenge
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
    /// Returns a new instance of AuthoredChallenge struct.
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

    /// Extracts fields for a struct from supplied JSON and fill up struct fields with those values
    pub fn from_json(&mut self, response_json: &Value) {
        // Extract single fields
        let id = response_json.get("id").unwrap().as_str().unwrap();
        self.id = id.to_string();
        let name = response_json.get("name").unwrap().as_str().unwrap();
        self.name = name.to_string();
        let description = response_json.get("description").unwrap().as_str().unwrap();
        self.description = description.to_string();
        let rank = response_json.get("rank").unwrap().as_i64().unwrap();
        self.rank = rank;
        let rank_name = response_json.get("rankName").unwrap().as_str().unwrap();
        self.rank_name = rank_name.to_string();

        // Extract tags
        let tags = response_json.get("tags").unwrap().as_array().unwrap();
        let mut tag_list: Vec<String> = Vec::new();
        for tag in tags {
            tag_list.push(tag.as_str().unwrap().to_string());
        }
        self.tags = tag_list;

        // Extract languages
        let languages = response_json.get("languages").unwrap().as_array().unwrap();
        let mut lang_list: Vec<String> = Vec::new();
        for language in languages {
            lang_list.push(language.as_str().unwrap().to_string());
        }
        self.languages = lang_list;
    }
}

impl Default for AuthoredChallenge {
    /// Returns a new instance of AuthoredChallenge struct with default values.
    fn default() -> Self {
        Self::new()
    }
}
