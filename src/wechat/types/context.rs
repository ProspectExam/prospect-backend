use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

use super::{*};

#[derive(Debug, Clone, Default)]
pub struct GlobalField {
  pub(crate) access_token: Option<String>,
  pub(crate) expired_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct Context {
  pub pool: PPool,
  pub options: Arc<Options>,
  pub global_field: Arc<Mutex<GlobalField>>,
  pub(crate) session: Option<Code2SessionResponse>,
}

impl Context {
  pub fn new(pool: PPool, options: Arc<Options>) -> Self {
    Context {
      pool,
      options,
      global_field: Arc::new(Mutex::new(GlobalField::default())),
      session: None,
    }
  }
}
