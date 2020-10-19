pub mod err;
pub mod user;
pub mod user_challenges;

pub mod codewars {

    use crate::err::Error;
    use crate::user::{OverallRank, User};
    use reqwest::StatusCode;
    use serde_json::Value;

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
                        // my_user.ranks.overall.languages
                        return Ok(my_user);
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
}
