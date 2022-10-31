#[derive(Copy, Clone, Debug)]
pub enum Error {
  // wechat defined error

  // 0
  Success,

  // -1
  WechatServerBusy,
  // 40001
  MismatchedAppSec,
  // 40002
  GetParamInvalid,
  // 40013
  InvalidAppId,
  // 40029
  InvalidCode,
  // 45011
  RequestTooFast,
  // 40226
  HighRiskUser,

  // self defined error

  /// Invalid Json from wechat server
  InvalidJson,
  /// Cannot connect to wechat server
  NetworkToWechatErr,
  /// Reply from wechat server not contains union_id
  UnionIdNotFound,
  /// Access token expired
  TokenExpired,
  /// Reply from wechat server not contains openid
  OpenIdNotFound,
  /// Reply from internal database error
  DatabaseErr,
  /// Unknown error
  UnknownErr,
}

impl Into<i32> for Error {
  fn into(self) -> i32 {
    match self {
      // wechat defined error
      Error::Success => 0,

      Error::WechatServerBusy => -1,
      Error::MismatchedAppSec => 40001,
      Error::GetParamInvalid => 40002,
      Error::InvalidAppId => 40013,
      Error::InvalidCode => 40029,
      Error::RequestTooFast => 45011,
      Error::HighRiskUser => 40226,

      // self defined error
      Error::InvalidJson => 101,
      Error::NetworkToWechatErr => 102,
      Error::UnionIdNotFound => 103,
      Error::TokenExpired => 104,
      Error::OpenIdNotFound => 105,
      Error::DatabaseErr => 106,
      Error::UnknownErr => 999,
    }
  }
}

impl Into<String> for Error {
  fn into(self) -> String {
    match self {
      // wechat defined error
      Error::Success => "success".into(),

      Error::WechatServerBusy => "wechat server busy".into(),
      Error::MismatchedAppSec => "mismatched app secret".into(),
      Error::GetParamInvalid => "get method provided invalid param".into(),
      Error::InvalidAppId => "invalid app id".into(),
      Error::InvalidCode => "invalid code".into(),
      Error::RequestTooFast => "request too fast".into(),
      Error::HighRiskUser => "high risk user".into(),

      // self defined error
      Error::InvalidJson => "invalid json".into(),
      Error::NetworkToWechatErr => "network to wechat error".into(),
      Error::UnionIdNotFound => "union id not found".into(),
      Error::TokenExpired => "access token expired".into(),
      Error::OpenIdNotFound => "open id not found".into(),
      Error::DatabaseErr => "database error".into(),
      Error::UnknownErr => "unknown error".into(),
    }
  }
}

impl From<i32> for Error {
  fn from(value: i32) -> Self {
    match value {
      // wechat defined error
      0 => Error::Success,

      -1 => Error::WechatServerBusy,
      40001 => Error::MismatchedAppSec,
      40002 => Error::GetParamInvalid,
      40013 => Error::InvalidAppId,
      40029 => Error::InvalidCode,
      45011 => Error::RequestTooFast,
      40226 => Error::HighRiskUser,

      // self defined error
      101 => Error::InvalidJson,
      102 => Error::NetworkToWechatErr,
      103 => Error::UnionIdNotFound,
      104 => Error::TokenExpired,
      105 => Error::OpenIdNotFound,
      106 => Error::DatabaseErr,
      _ => Error::UnknownErr,
    }
  }
}
