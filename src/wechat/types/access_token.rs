use serde::{Serialize, Deserialize};
use chrono::{prelude::*, Duration};
use crypto::digest::Digest;

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessToken {
  token: String,
  expired: DateTime<Utc>,
}

impl AccessToken {
  pub fn new(union_id: &str) -> Self {
    let now_utc: DateTime<Utc> = Utc::now();
    let expired_utc = now_utc + Duration::days(3);
    let mut hasher = crypto::sha3::Sha3::sha3_256();
    hasher.input_str(union_id);
    AccessToken {
      token: hasher.result_str(),
      expired: expired_utc,
    }
  }

  pub fn empty() -> Self {
    AccessToken {
      token: "".to_string(),
      expired: DateTime::default(),
    }
  }

  pub fn is_valid(&self) -> bool {
    !self.token.is_empty()
  }
}

impl From<String> for AccessToken {
  fn from(value: String) -> Self {
    AccessToken {
      token: value,
      expired: Utc::now() + Duration::days(3),
    }
  }
}

impl Into<String> for AccessToken {
  fn into(self) -> String {
    self.token
  }
}