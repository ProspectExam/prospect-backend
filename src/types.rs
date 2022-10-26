use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpInfo {
  pub username: String,
  pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpResult {
  pub success: bool,
  pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LogInInfo {
  pub username: String,
  pub password: String,
}

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
