use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PostContent {
  // TODO: more detail
  pub title: String,
  pub date: String,
  pub author: String,
  pub content: String,
}
