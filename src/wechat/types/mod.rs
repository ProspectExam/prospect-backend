use std::sync::Arc;

use serde::{Serialize, Deserialize};
use argh::FromArgs;
use crate::users::ProspectSqlPool;

pub type PPool = Arc<tokio::sync::Mutex<ProspectSqlPool>>;

/// serve_wx param parse
#[derive(Debug, FromArgs)]
pub struct Options {
  /// bind addr
  #[argh(positional)]
  pub addr: String,

  /// cert file
  #[argh(option, short = 'c')]
  pub cert: String,

  /// key file
  #[argh(option, short = 'k')]
  pub key: String,

  /// sql user
  #[argh(option, short = 'u')]
  pub sql_user: String,

  /// sql ip
  #[argh(option, short = 'a')]
  pub sql_addr: String,

  /// sql passwd
  #[argh(option, short = 'p')]
  pub sql_passwd: String,

  /// wx appid
  #[argh(option, short = 'i')]
  pub wx_appid: String,

  /// wx appsecret
  #[argh(option, short = 's')]
  pub wx_appsecret: String,
}

/***********************************************/
// mod include
mod code;
mod waterfall;
mod subscribe;
mod post;
mod source;

mod error;

/***********************************************/
// export
pub use code::*;
pub use waterfall::*;
pub use subscribe::*;
pub use post::*;
pub use source::*;

pub use error::*;

/***********************************************/
// mini-program-server communication api

#[derive(Debug, Clone)]
pub struct Context {
  pub pool: PPool,
  pub options: Arc<Options>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessToken;

impl AccessToken {
  pub fn new() -> Self {
    AccessToken
  }
}

impl AccessToken {
  pub fn empty() -> Self {
    // TODO:
    AccessToken
  }

  pub fn is_valid(&self) -> bool {
    true
  }
}

impl From<String> for AccessToken {
  fn from(value: String) -> Self {
    // TODO: todo!()
    return AccessToken;
  }
}

impl Into<String> for AccessToken {
  fn into(self) -> String {
    // TODO: todo!()
    "1234567890".to_string()
  }
}

/***********************************************/
// wechart server json definitions

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Code2SessionResponse {
  pub(crate) openid: Option<String>,
  pub(crate) session_key: Option<String>,
  pub(crate) unionid: Option<String>,
  pub(crate) errcode: Option<i32>,
  pub(crate) errmsg: Option<String>,
}

