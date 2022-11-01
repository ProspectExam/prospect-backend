use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use serde::{Serialize, Deserialize};
use argh::FromArgs;
use crate::database::ProspectSqlPool;

// pub type PPool = Arc<tokio::sync::Mutex<ProspectSqlPool>>;
pub type PPool = ProspectSqlPool;

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

mod access_token;
mod context;

/***********************************************/
// export
pub use code::*;
pub use waterfall::*;
pub use subscribe::*;
pub use post::*;
pub use source::*;

pub use error::*;

pub use access_token::*;
pub use context::*;

/***********************************************/
// mini-program-server communication api


/***********************************************/
// wechart server json definitions

/// Code2Session response json struct.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Code2SessionResponse {
  pub(crate) openid: Option<String>,
  pub(crate) session_key: Option<String>,
  pub(crate) errcode: Option<i32>,
  pub(crate) errmsg: Option<String>,
}

/// getAccessToken response json struct.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct GetAccessTokenResponse {
  pub(crate) access_token: Option<String>,
  pub(crate) expires_in: Option<u32>,
  pub(crate) errcode: Option<i32>,
  pub(crate) errmsg: Option<String>,
}
