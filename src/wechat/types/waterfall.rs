use crate::wechat::types::Error;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFall {
  pub err_code: i32,
  pub message: String,
  pub items: Vec<WaterFallItem>,
}

impl WaterFall {
  pub fn new(arg: Result<Vec<WaterFallItem>, Error>) -> Self {
    match arg {
      Ok(items) => WaterFall {
        err_code: 0,
        message: "".to_string(),
        items,
      },
      Err(e) => WaterFall {
        err_code: e.into(),
        message: e.into(),
        items: Vec::new(),
      },
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFallItem {
  pub img_source_link: String,
  pub title: String,
  pub post_id: String,
}

impl WaterFallItem {
  pub fn new(img_source_link: String, title: String, post_id: String) -> Self {
    WaterFallItem {
      img_source_link,
      title,
      post_id,
    }
  }
}
