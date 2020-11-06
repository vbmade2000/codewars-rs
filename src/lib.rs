pub mod code_challenges;
pub mod err;
pub mod user;
pub mod user_challenges;

pub mod codewars {

    use crate::code_challenges::CodeChallenge;
    use crate::err::Error;
    use crate::user::{OverallRank, User};
    use crate::user_challenges::{AuthoredChallenge, CompletedChallenge};
    use reqwest::StatusCode;
    use serde_json::Value;
    use std::vec::Vec;

    /// A main structure that contains all the methods.
    pub struct Codewars {
        pub token: String,
    }

    // Methods for Codewars struct
    impl Codewars {
        /// Returns a new instance of Codewars struct.
        pub fn new(token: String) -> Self {
            Codewars { token }
        }

        /// Retrieve a single user information from Codewars REST API. This method doesn't require use of token.
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
                            StatusCode::NOT_FOUND => Err(Error::UserNotFound { username }),
                            _ => Err(Error::CodewarsError {
                                message: "Error in retrieving data".to_string(),
                            }),
                        }
                    }
                }
                Err(e) => Err(Error::ReqwestError { source: e }),
            }
        }

        /// Retrieves all the completed challenges.
        pub fn get_completed_challenges(
            username: String,
        ) -> Result<Vec<CompletedChallenge>, Error> {
            let mut current_page = 0;
            let mut total_pages: Option<u64> = None;
            let mut completed_challenges: Vec<CompletedChallenge> = Vec::new();
            loop {
                if total_pages.is_some() && current_page >= total_pages.unwrap() {
                    break;
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
                            if total_pages.is_none() {
                                total_pages = Some(total_pages_received);
                            }
                            for d in data {
                                let mut completed_challenge = CompletedChallenge::new();
                                completed_challenge.from_json(d);
                                completed_challenges.push(completed_challenge);
                            }
                            current_page += 1;
                        // Ok(completed_challenges)
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

        /// Returns all the authored challenges.
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
                        for authored_challenge_received in authored_challenges_received {
                            let mut authored_challenge = AuthoredChallenge::new();
                            authored_challenge.from_json(authored_challenge_received);
                            authored_challenges.push(authored_challenge);
                        }
                        Ok(authored_challenges)
                    } else {
                        match response.status() {
                            StatusCode::NOT_FOUND => Err(Error::UserNotFound { username }),
                            _ => Err(Error::CodewarsError {
                                message: "Error in retrieving data".to_string(),
                            }),
                        }
                    }
                }
                Err(e) => Err(Error::ReqwestError { source: e }),
            }
        }

        /// Returns a single code challenge detail.
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
                        code_challenge.from_json(&response_json);

                        Ok(code_challenge)
                    } else {
                        match response.status() {
                            StatusCode::NOT_FOUND => {
                                Err(Error::ChallengeNotFound { challenge_title })
                            }
                            _ => Err(Error::CodewarsError {
                                message: "Error in retrieving data".to_string(),
                            }),
                        }
                    }
                }
                Err(e) => Err(Error::ReqwestError { source: e }),
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
        let _completed_challenges = Codewars::get_completed_challenges("hobovsky".to_string());
        // assert_eq!(_completed_challenges.unwrap().len(), 878);
    }

    #[test]
    fn test_get_authored_challenges() {
        let _authored_challenges = Codewars::get_authored_challenges("hobovsky".to_string());
        // assert_eq!(_authoered_challenges.unwrap().len(), 878);
    }

    #[test]
    fn test_get_code_challenge() {
        let _code_challenge = Codewars::get_code_challenge("valid-braces".to_string());
        // assert_eq!(_code_challenge.unwrap().rank.id, -6);
        // assert_eq!(_code_challenge.unwrap().unresolved.issues, 12);
        assert_eq!(
            _code_challenge.unwrap().created_by.username,
            "xDranik".to_string()
        );
    }
}
