use super::{AccessToken, Error};

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeInfo {
  pub code: String,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeResult {
  pub err_code: i32,
  pub message: String,
  pub access_token: String,
}

impl CodeResult {
  pub fn new(arg: Result<AccessToken, Error>) -> Self {
    match arg {
      Ok(token) => CodeResult {
        err_code: Error::Success.into(),
        message: Error::Success.into(),
        access_token: token.into(),
      },
      Err(e) => CodeResult {
        err_code: e.into(),
        message: e.into(),
        access_token: "".into(),
      },
    }
  }
}
