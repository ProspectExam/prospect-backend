use serde::{Serialize, Deserialize};
use chrono::{prelude::*, Duration};
use crypto::digest::Digest;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccessToken {
  pub token: String,
  pub expired: DateTime<Utc>,
}

impl AccessToken {
  /// Generate an access token from open_id and current timestamp.
  pub fn new(open_id: &str) -> Self {
    let now_utc: DateTime<Utc> = Utc::now();
    let expired_utc = now_utc + Duration::days(3);
    let mut hasher = crypto::sha3::Sha3::sha3_256();
    let b = open_id.as_bytes()
      .iter()
      .chain(now_utc.timestamp().to_string().as_bytes().iter())
      .map(|x| *x)
      .collect::<Vec<u8>>();
    hasher.input(&b);
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