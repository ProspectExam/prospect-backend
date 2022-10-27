use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeInfo {
  pub school_code: Vec<u32>,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeResult {
  pub err_code: i32,
  pub message: String,
}
