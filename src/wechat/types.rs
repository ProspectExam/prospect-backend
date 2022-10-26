use std::sync::Arc;
use std::convert::Infallible;

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

#[derive(Debug, Clone)]
pub struct Context {
  pub pool: PPool,
  pub options: Arc<Options>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeInfo {
  pub code: String,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeResult {
  pub success: bool,
  pub message: String,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFall {
  pub success: bool,
  pub message: String,
  pub items: Vec<WaterFallItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WaterFallItem {
  pub img_source_link: String,
  pub title: String,
  pub post_id: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeInfo {
  pub school_code: Vec<u32>,
  pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeResult {
  pub success: bool,
  pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostContent {
  // TODO: more detail
  pub title: String,
  pub date: String,
  pub author: String,
  pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceContent {
  pub success: bool,
  pub message: String,
  pub content: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Code2SessionResponse {
  pub(crate) openid: String,
  pub(crate) session_key: String,
  pub(crate) unionid: String,
  pub(crate) errcode: i32,
  pub(crate) errmsg: String,
}
