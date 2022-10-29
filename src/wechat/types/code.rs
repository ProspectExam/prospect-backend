use super::{AccessToken, Error};

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeInfo {
  pub code: String,
  pub open_id: String,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeResult {
  pub err_code: i32,
  pub message: String,
  pub open_id: String,
  pub access_token: String,
}

impl CodeResult {
  /// init a new CodeResult with open id and access token.
  pub fn new(arg: Result<(String, AccessToken), Error>) -> Self {
    match arg {
      Ok(ctx) => CodeResult {
        err_code: Error::Success.into(),
        message: Error::Success.into(),
        open_id: ctx.0,
        access_token: ctx.1.into(),
      },
      Err(e) => CodeResult {
        err_code: e.into(),
        message: e.into(),
        open_id: "".into(),
        access_token: "".into(),
      },
    }
  }
}
