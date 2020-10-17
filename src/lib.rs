pub mod user;

pub mod codewars {

    use crate::user::User;
    use serde_json::Value;
    use std::str::FromStr;

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
        pub fn get_user(username: String) {
            let url = format!("https://www.codewars.com/api/v1/users/{}", username);

            // Call the URL
            let user_result = reqwest::blocking::get(&url);

            // Process the result
            match user_result {
                Ok(response) => {
                    let mut my_user = User::new();
                    if response.status().is_success() {
                        let user_json: Value = response.json().unwrap();
                        my_user.name =
                            String::from(user_json.get("name").unwrap().as_str().unwrap());
                        my_user.username =
                            String::from(user_json.get("username").unwrap().as_str().unwrap());
                        println!("Got response")
                    }
                }
                Err(e) => println!("Got error"),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::codewars::Codewars;
    use crate::user::User;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_user_struct() {
        let test_user = User::new();
        Codewars::get_user("vbmade2000".to_string());
    }
}
