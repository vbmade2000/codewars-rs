pub mod code_challenges;
pub mod err;
pub mod user;
pub mod user_challenges;

pub mod codewars {

    use crate::code_challenges::{CodeChallenge, CreatedBy, Rank, Unresolved};
    use crate::err::Error;
    use crate::user::{OverallRank, User};
    use crate::user_challenges::{AuthoredChallenge, CompletedChallenge};
    use reqwest::StatusCode;
    use serde_json::Value;
    use std::vec::Vec;

    // Main struct that contains all the methods
    pub struct Codewars {
        pub token: String,
    }

    // Methods for Codewars struct
    impl Codewars {
        pub fn new(token: String) -> Self {
            Codewars { token }
        }

        /// Retrieve user information. This method doesn't require use of token.
        pub fn get_user(username: String) -> Result<User, Error> {
            let url = format!("https://www.codewars.com/api/v1/users/{}", username);

            // Call the URL
            let user_result = reqwest::blocking::get(&url);

            // Process the result
            match user_result {
                Ok(response) => {
                    let mut my_user = User::new();
                    // We create User instance only if user details successfully retrieved
                    if response.status().is_success() {
                        let user_json: Value = response.json().unwrap();
                        my_user.name =
                            String::from(user_json.get("name").unwrap().as_str().unwrap());
                        my_user.username =
                            String::from(user_json.get("username").unwrap().as_str().unwrap());
                        my_user.clan = Some(String::from(
                            user_json.get("clan").unwrap().as_str().unwrap(),
                        ));
                        my_user.honor = user_json.get("clan").unwrap().as_i64();
                        my_user.leaderboard_position =
                            user_json.get("leaderboardPosition").unwrap().as_i64();

                        let user_skills = user_json.get("skills").unwrap().as_array();

                        // Fill up skills vector
                        if user_skills.is_some() {
                            for skill in user_skills.unwrap() {
                                my_user.skills.push(String::from(skill.as_str().unwrap()));
                            }
                        }

                        // Fill up CodeChallenges struct
                        let user_codechallanges = user_json
                            .get("codeChallenges")
                            .unwrap()
                            .as_object()
                            .unwrap();
                        my_user.code_challenges.total_authored = user_codechallanges
                            .get("totalAuthored")
                            .unwrap()
                            .as_u64()
                            .unwrap();
                        my_user.code_challenges.total_completed = user_codechallanges
                            .get("totalCompleted")
                            .unwrap()
                            .as_u64()
                            .unwrap();

                        /* -------- Fill up ranks struct */
                        let user_ranks = user_json.get("ranks").unwrap().as_object().unwrap();
                        // Fill up ranks->overall struct
                        let overall_rank = user_ranks.get("overall").unwrap().as_object().unwrap();
                        my_user.ranks.overall.rank =
                            overall_rank.get("rank").unwrap().as_i64().unwrap();
                        my_user.ranks.overall.name =
                            String::from(overall_rank.get("name").unwrap().as_str().unwrap());
                        my_user.ranks.overall.color =
                            String::from(overall_rank.get("color").unwrap().as_str().unwrap());
                        my_user.ranks.overall.score =
                            overall_rank.get("score").unwrap().as_u64().unwrap();

                        // Fill up ranks->languages struct
                        let user_languages =
                            user_ranks.get("languages").unwrap().as_object().unwrap();
                        for lang in user_languages {
                            let rank = lang.1.get("rank").unwrap().as_i64().unwrap();
                            let name = String::from(lang.1.get("name").unwrap().as_str().unwrap());
                            let color =
                                String::from(lang.1.get("color").unwrap().as_str().unwrap());
                            let score = lang.1.get("score").unwrap().as_u64().unwrap();
                            let mut temp_overall_rank = OverallRank::new();
                            temp_overall_rank.rank = rank;
                            temp_overall_rank.name = name;
                            temp_overall_rank.color = color;
                            temp_overall_rank.score = score;

                            my_user
                                .ranks
                                .languages
                                .insert(lang.0.clone(), temp_overall_rank);
                        }
                        Ok(my_user)
                    } else {
                        match response.status() {
                            StatusCode::NOT_FOUND => return Err(Error::UserNotFound { username }),
                            _ => {
                                return Err(Error::CodewarsError {
                                    message: "Error in retrieving data".to_string(),
                                })
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(Error::ReqwestError { source: e });
                }
            }
        }

        pub fn get_completed_challenges(
            username: String,
        ) -> Result<Vec<CompletedChallenge>, Error> {
            let mut current_page = 0;
            let mut total_pages: Option<u64> = None;
            let mut completed_challenges: Vec<CompletedChallenge> = Vec::new();
            loop {
                if total_pages.is_some() {
                    if current_page >= total_pages.unwrap() {
                        break;
                    }
                }
                let url = format!(
                    "https://www.codewars.com/api/v1/users/{}/code-challenges/completed?page={}",
                    username, current_page
                );
                let result = reqwest::blocking::get(&url);
                match result {
                    Ok(response) => {
                        if response.status().is_success() {
                            let json_data: Value = response.json().unwrap();
                            let total_pages_received: u64 =
                                json_data.get("totalPages").unwrap().as_u64().unwrap();
                            let data = json_data.get("data").unwrap().as_array().unwrap();
                            let total_items_received: u64 =
                                json_data.get("totalItems").unwrap().as_u64().unwrap();
                            if total_pages.is_none() {
                                total_pages = Some(total_pages_received);
                            }
                            for d in data {
                                let challenge_id = d.get("id").unwrap().as_str().unwrap();
                                let challenge_name = d.get("name").unwrap().as_str().unwrap();
                                let challenge_slug = d.get("slug").unwrap().as_str().unwrap();
                                let challenge_completed_at =
                                    d.get("completedAt").unwrap().as_str().unwrap();
                                let completed_languages =
                                    d.get("completedLanguages").unwrap().as_array().unwrap();
                                let mut completed_challenge = CompletedChallenge::new();
                                completed_challenge.id = String::from(challenge_id);
                                completed_challenge.name = String::from(challenge_name);
                                completed_challenge.slug = String::from(challenge_slug);
                                completed_challenge.completed_at =
                                    String::from(challenge_completed_at);
                                for completed_language in completed_languages {
                                    completed_challenge
                                        .completed_languages
                                        .push(String::from(completed_language.as_str().unwrap()));
                                }
                                completed_challenges.push(completed_challenge);
                            }
                            current_page += 1;
                        // return Ok(completed_challenges);
                        } else {
                            match response.status() {
                                StatusCode::NOT_FOUND => {
                                    return Err(Error::UserNotFound { username })
                                }
                                _ => {
                                    return Err(Error::CodewarsError {
                                        message: "Error in retrieving data".to_string(),
                                    })
                                }
                            }
                        }
                    }
                    Err(e) => return Err(Error::ReqwestError { source: e }),
                }
                // current_page += 1;
            }
            Ok(completed_challenges)
        }

        pub fn get_authored_challenges(username: String) -> Result<Vec<AuthoredChallenge>, Error> {
            let mut authored_challenges: Vec<AuthoredChallenge> = Vec::new();
            let url = format!(
                "https://www.codewars.com/api/v1/users/{}/code-challenges/authored",
                username
            );
            let result = reqwest::blocking::get(&url);
            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        let json_data: Value = response.json().unwrap();
                        let authored_challenges_received =
                            json_data.get("data").unwrap().as_array().unwrap();
                        for authored_challenge_reeived in authored_challenges_received {
                            // Get Values
                            let id = authored_challenge_reeived
                                .get("id")
                                .unwrap()
                                .as_str()
                                .unwrap();
                            let name = authored_challenge_reeived
                                .get("name")
                                .unwrap()
                                .as_str()
                                .unwrap();
                            let description = authored_challenge_reeived
                                .get("description")
                                .unwrap()
                                .as_str()
                                .unwrap();
                            let rank = authored_challenge_reeived
                                .get("rank")
                                .unwrap()
                                .as_i64()
                                .unwrap();
                            let rank_name = authored_challenge_reeived
                                .get("rankName")
                                .unwrap()
                                .as_str()
                                .unwrap();
                            let tags = authored_challenge_reeived
                                .get("tags")
                                .unwrap()
                                .as_array()
                                .unwrap();
                            let languages = authored_challenge_reeived
                                .get("languages")
                                .unwrap()
                                .as_array()
                                .unwrap();

                            // Create and fill Vector for tags
                            let mut tag_list: Vec<String> = Vec::new();
                            for tag in tags {
                                tag_list.push(tag.as_str().unwrap().to_string());
                            }

                            // Create and fill Vector for languages
                            let mut language_list: Vec<String> = Vec::new();
                            for language in languages {
                                language_list.push(language.as_str().unwrap().to_string());
                            }

                            let mut authored_challenge = AuthoredChallenge::new();
                            authored_challenge.id = id.to_string();
                            authored_challenge.name = name.to_string();
                            authored_challenge.description = description.to_string();
                            authored_challenge.rank = rank;
                            authored_challenge.rank_name = rank_name.to_string();
                            authored_challenge.tags = tag_list;
                            authored_challenge.languages = language_list;
                            authored_challenges.push(authored_challenge);
                        }
                        return Ok(authored_challenges);
                    } else {
                        match response.status() {
                            StatusCode::NOT_FOUND => return Err(Error::UserNotFound { username }),
                            _ => {
                                return Err(Error::CodewarsError {
                                    message: "Error in retrieving data".to_string(),
                                })
                            }
                        }
                    }
                }
                Err(e) => return Err(Error::ReqwestError { source: e }),
            }
        }

        pub fn get_code_challenge(challenge_title: String) -> Result<CodeChallenge, Error> {
            let url = format!(
                "https://www.codewars.com/api/v1/code-challenges/{}",
                challenge_title
            );
            let result = reqwest::blocking::get(&url);

            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        let mut code_challenge = CodeChallenge::new();
                        let response_json: Value = response.json().unwrap();

                        // Extract single values
                        let id = response_json
                            .get("id")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let name = response_json
                            .get("name")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let slug = response_json
                            .get("slug")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let category = response_json
                            .get("category")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let published_at = response_json
                            .get("publishedAt")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let approved_at = response_json
                            .get("approvedAt")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();

                        // Extract Languages
                        let languages_json =
                            response_json.get("languages").unwrap().as_array().unwrap();
                        let mut languages: Vec<String> = Vec::new();
                        for lang in languages_json {
                            languages.push(lang.as_str().unwrap().to_string());
                        }

                        let url = response_json
                            .get("url")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();

                        // Extract Rank details
                        let rank_json = response_json.get("rank").unwrap().as_object().unwrap();
                        let rank_id = rank_json.get("id").unwrap().as_i64().unwrap();
                        let rank_name =
                            rank_json.get("name").unwrap().as_str().unwrap().to_string();
                        let rank_color = rank_json
                            .get("color")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let mut rank = Rank::new();
                        rank.id = rank_id;
                        rank.name = rank_name;
                        rank.color = rank_color;

                        let created_at = response_json
                            .get("createdAt")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();

                        // Extract CreatedBy details
                        let created_by_json =
                            response_json.get("createdBy").unwrap().as_object().unwrap();
                        let created_by_username = created_by_json
                            .get("username")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let created_by_url = created_by_json
                            .get("url")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let mut created_by = CreatedBy::new();
                        created_by.username = created_by_username;
                        created_by.url = created_by_url;

                        // Extract ApprovedBy details
                        let approved_by_json = response_json
                            .get("approvedBy")
                            .unwrap()
                            .as_object()
                            .unwrap();
                        let approved_by_username = approved_by_json
                            .get("username")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let approved_by_url = approved_by_json
                            .get("url")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let mut approved_by = CreatedBy::new();
                        approved_by.username = approved_by_username;
                        approved_by.url = approved_by_url;

                        let description = response_json
                            .get("description")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let total_attempts = response_json
                            .get("totalAttempts")
                            .unwrap()
                            .as_u64()
                            .unwrap();
                        let total_completed = response_json
                            .get("totalCompleted")
                            .unwrap()
                            .as_u64()
                            .unwrap();
                        let total_stars =
                            response_json.get("totalStars").unwrap().as_u64().unwrap();
                        let vote_score = response_json.get("voteScore").unwrap().as_u64().unwrap();
                        let contributors_wanted = response_json
                            .get("contributorsWanted")
                            .unwrap()
                            .as_bool()
                            .unwrap();

                        // Extract tags detail
                        let tags_json = response_json.get("tags").unwrap().as_array().unwrap();
                        let mut tags: Vec<String> = Vec::new();
                        for tg in tags_json {
                            tags.push(tg.as_str().unwrap().to_string());
                        }

                        // Extract Unresolved details
                        let unresolved_json = response_json
                            .get("unresolved")
                            .unwrap()
                            .as_object()
                            .unwrap();
                        let unresolved_issues =
                            unresolved_json.get("issues").unwrap().as_u64().unwrap();
                        let unresolved_suggestions = unresolved_json
                            .get("suggestions")
                            .unwrap()
                            .as_u64()
                            .unwrap();
                        let mut unresolved = Unresolved::new();
                        unresolved.issues = unresolved_issues;
                        unresolved.suggestions = unresolved_suggestions;

                        // Fill up CodeChallenge struct
                        code_challenge.id = id;
                        code_challenge.name = name;
                        code_challenge.slug = slug;
                        code_challenge.category = category;
                        code_challenge.approved_at = approved_at;
                        code_challenge.published_at = published_at;
                        code_challenge.languages = languages;
                        code_challenge.url = url;
                        code_challenge.rank = rank;
                        code_challenge.created_at = created_at;
                        code_challenge.created_by = created_by;
                        code_challenge.approved_by = approved_by;
                        code_challenge.description = description;
                        code_challenge.total_attempts = total_attempts;
                        code_challenge.total_completed = total_completed;
                        code_challenge.total_stars = total_stars;
                        code_challenge.vote_score = vote_score;
                        code_challenge.tags = tags;
                        code_challenge.contributors_wanted = contributors_wanted;
                        code_challenge.unresolved = unresolved;

                        Ok(code_challenge)
                    } else {
                        match response.status() {
                            StatusCode::NOT_FOUND => {
                                return Err(Error::ChallengeNotFound {
                                    challenge_title: challenge_title,
                                })
                            }
                            _ => Err(Error::CodewarsError {
                                message: "Error in retrieving data".to_string(),
                            }),
                        }
                    }
                }
                Err(e) => return Err(Error::ReqwestError { source: e }),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::codewars::Codewars;
    // use crate::user::User;

    #[test]
    fn test_get_struct() {
        // Call a function
        let user = Codewars::get_user("vbmade2000".to_string()).unwrap();

        // Assert values
        assert_eq!(user.name, "Malhar Vora".to_string());
        assert_eq!(user.username, "vbmade2000".to_string());
    }

    #[test]
    fn test_get_completed_challenges() {
        let completed_challenges = Codewars::get_completed_challenges("hobovsky".to_string());
        // assert_eq!(completed_challenges.unwrap().len(), 878);
    }

    #[test]
    fn test_get_authored_challenges() {
        let _authored_challenges = Codewars::get_authored_challenges("hobovsky".to_string());
        // assert_eq!(authoered_challenges.unwrap().len(), 878);
    }

    #[test]
    fn test_get_code_challenge() {
        let _code_challenge = Codewars::get_code_challenge("valid-braces".to_string());
    }
}
