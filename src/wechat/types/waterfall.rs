use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFall {
  pub err_code: i32,
  pub message: String,
  pub items: Vec<WaterFallItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFallItem {
  pub img_source_link: String,
  pub title: String,
  pub post_id: u64,
}
