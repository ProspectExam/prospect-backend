use reqwest::get;

use serde::{Serialize, Deserialize};

use crate::wechat::common::ErrKind::{Connect, ParseJson};
use super::types::*;

pub enum ErrKind {
  Connect,
  ParseJson,
}

impl Into<String> for ErrKind {
  fn into(self) -> String {
    match self {
      Connect => "contact to wechat server error".into(),
      ParseJson => "invalid data from wechat server".into(),
    }
  }
}

pub async fn get_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, ErrKind> {
  match get(url).await {
    Ok(r) => {
      match r.json::<T>().await {
        Ok(j) => {
          Ok(j)
        }
        Err(_) => {
          Err(ParseJson)
        }
      }
    }
    Err(_) => {
      Err(Connect)
    }
  }
}
