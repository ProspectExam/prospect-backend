use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceContent {
  pub err_code: i32,
  pub message: String,
  pub content: Vec<u8>,
}
