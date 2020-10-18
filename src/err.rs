use reqwest::Error as ReqError;
use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("{}", source))]
    ReqwestError { source: ReqError },

    #[snafu(display("Error in retriving user data"))]
    CodewarsError { message: String },

    #[snafu(display("User {} not found", username))]
    UserNotFound { username: String },
}
