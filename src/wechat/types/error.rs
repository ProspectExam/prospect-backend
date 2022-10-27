#[derive(Copy, Clone, Debug)]
pub enum Error {
  // wechat defined error
  Success,

  WechatServerBusy,
  InvalidCode,
  RequestTooFast,
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
  /// Unknown error
  UnknownErr,
}

impl Into<i32> for Error {
  fn into(self) -> i32 {
    match self {
      // wechat defined error
      Error::Success => 0,

      Error::WechatServerBusy => -1,
      Error::InvalidCode => 40029,
      Error::RequestTooFast => 45011,
      Error::HighRiskUser => 40226,

      // self defined error
      Error::InvalidJson => 101,
      Error::NetworkToWechatErr => 102,
      Error::UnionIdNotFound => 103,
      Error::TokenExpired => 104,
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
      Error::InvalidCode => "invalid code".into(),
      Error::RequestTooFast => "request too fast".into(),
      Error::HighRiskUser => "high risk user".into(),

      // self defined error
      Error::InvalidJson => "invalid json".into(),
      Error::NetworkToWechatErr => "network to wechat error".into(),
      Error::UnionIdNotFound => "union id not found".into(),
      Error::TokenExpired => "access token expired".into(),
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
      40029 => Error::InvalidCode,
      45011 => Error::RequestTooFast,
      40226 => Error::HighRiskUser,

      // self defined error
      101 => Error::InvalidJson,
      102 => Error::NetworkToWechatErr,
      103 => Error::UnionIdNotFound,
      104 => Error::TokenExpired,
      _ => Error::UnknownErr,
    }
  }
}
