use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeInfo {
  pub code: String,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeResult {
  pub success: bool,
  pub message: String,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFall {
  pub success: bool,
  pub message: String,
  pub items: Vec<WaterFallItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFallItem {
  pub img_source_link: String,
  pub title: String,
  pub post_id: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeInfo {
  pub school_code: Vec<u32>,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeResult {
  pub success: bool,
  pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostContent {
  // TODO: more detail
  pub title: String,
  pub date: String,
  pub author: String,
  pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceContent {
  pub success: bool,
  pub message: String,
  pub content: Vec<u8>,
}
