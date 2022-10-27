use serde::{Serialize, Deserialize};

/// /send_code receive
#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpInfo {
  pub username: String,
  pub password: String,
}

/// /send_code return
#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpResult {
  pub success: bool,
  pub message: String,
}

/// /log_in receive
#[derive(Deserialize, Serialize, Debug)]
pub struct LogInInfo {
  pub username: String,
  pub password: String,
}

/// /log_in return
#[derive(Deserialize, Serialize, Debug)]
pub struct LogInResult {
  pub success: bool,
  pub message: String,
  pub user_id: u32,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessToken;

impl Into<String> for AccessToken {
  fn into(self) -> String {
    // todo!()
    "".to_string()
  }
}
