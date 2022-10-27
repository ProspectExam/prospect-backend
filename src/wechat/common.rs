use reqwest::get;

use serde::{Serialize, Deserialize};

use super::types::*;

pub async fn get_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Error> {
  match get(url).await {
    Ok(r) => {
      match r.json::<T>().await {
        Ok(j) => {
          Ok(j)
        }
        Err(_) => {
          Err(Error::InvalidJson)
        }
      }
    }
    Err(_) => {
      Err(Error::NetworkToWechatErr)
    }
  }
}
