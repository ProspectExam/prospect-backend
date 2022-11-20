use serde::{Serialize, Deserialize};
use super::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeInfo {
  // pub school_code: u32,
  // pub department_code: u32,
  pub open_id: String,
  pub access_token: String,
  pub info: Vec<SubscribeDetail>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeDetail {
  pub school_code: u32,
  pub department_code: u32,
  pub oper: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeResult {
  pub err_code: i32,
  pub message: String,
}

impl SubscribeResult {
  pub fn new(arg: Result<(), Error>) -> Self {
    match arg {
      Ok(_) => SubscribeResult {
        err_code: 0,
        message: "".to_string(),
      },
      Err(e) => SubscribeResult {
        err_code: e.into(),
        message: e.into(),
      },
    }
  }
}
