use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum Error {
  // wechat defined error

  // 0
  Success,

  // -1
  SystemBusy,
  // 40001
  InvalidCredential,
  // 40002
  GetParamInvalid,
  // 40003
  ToUserOrOpenIdInvalid,
  // 40013
  InvalidAppId,
  // 40029
  InvalidCode,
  // 40037
  TemplateIdInvalid,
  // 41030
  PagePathInvalid,
  // 43101
  NeedSubscribe,
  // 45011
  ApiFrequencyLimit,
  // 47003
  TemplateParamAmbiguous,
  // 40226
  HighRiskUser,

  // self defined error

  /// Invalid Json from wechat server
  InvalidJsonFromWechat,
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
  /// Json request from miniprogram invalid
  InvalidJsonRequest,
  /// Unknown error
  UnknownErr,
}

impl Into<i32> for Error {
  fn into(self) -> i32 {
    match self {
      // wechat defined error
      Error::Success => 0,

      Error::SystemBusy => -1,
      Error::InvalidCredential => 40001,
      Error::GetParamInvalid => 40002,
      Error::ToUserOrOpenIdInvalid => 40003,
      Error::InvalidAppId => 40013,
      Error::InvalidCode => 40029,
      Error::TemplateIdInvalid => 40037,
      Error::PagePathInvalid => 41030,
      Error::NeedSubscribe => 43101,
      Error::ApiFrequencyLimit => 45011,
      Error::TemplateParamAmbiguous => 47003,
      Error::HighRiskUser => 40226,

      // self defined error
      Error::InvalidJsonFromWechat => 101,
      Error::NetworkToWechatErr => 102,
      Error::UnionIdNotFound => 103,
      Error::TokenExpired => 104,
      Error::OpenIdNotFound => 105,
      Error::DatabaseErr => 106,
      Error::InvalidJsonRequest => 107,
      Error::UnknownErr => 999,
    }
  }
}

impl Into<String> for Error {
  fn into(self) -> String {
    match self {
      // wechat defined error
      Error::Success => "success".into(),

      Error::SystemBusy => "wechat server busy".into(),
      Error::InvalidCredential => "mismatched app secret".into(),
      Error::GetParamInvalid => "get method provided invalid param".into(),
      Error::ToUserOrOpenIdInvalid => "to user or open id invalid".into(),
      Error::InvalidAppId => "invalid app id".into(),
      Error::InvalidCode => "invalid code".into(),
      Error::TemplateIdInvalid => "invalid template id".into(),
      Error::PagePathInvalid => "invalid page path".into(),
      Error::NeedSubscribe => "need subscribe".into(),
      Error::ApiFrequencyLimit => "request too fast".into(),
      Error::TemplateParamAmbiguous => "template param ambiguous".into(),
      Error::HighRiskUser => "high risk user".into(),

      // self defined error
      Error::InvalidJsonFromWechat => "invalid json".into(),
      Error::NetworkToWechatErr => "network to wechat error".into(),
      Error::UnionIdNotFound => "union id not found".into(),
      Error::TokenExpired => "access token expired".into(),
      Error::OpenIdNotFound => "open id not found".into(),
      Error::DatabaseErr => "database error".into(),
      Error::InvalidJsonRequest => "invalid json request".into(),
      Error::UnknownErr => "unknown error".into(),
    }
  }
}

impl From<i32> for Error {
  fn from(value: i32) -> Self {
    match value {
      // wechat defined error
      0 => Error::Success,

      -1 => Error::SystemBusy,
      40001 => Error::InvalidCredential,
      40002 => Error::GetParamInvalid,
      40003 => Error::ToUserOrOpenIdInvalid,
      40013 => Error::InvalidAppId,
      40029 => Error::InvalidCode,
      40037 => Error::TemplateIdInvalid,
      41030 => Error::PagePathInvalid,
      43101 => Error::NeedSubscribe,
      45011 => Error::ApiFrequencyLimit,
      47003 => Error::TemplateParamAmbiguous,
      40226 => Error::HighRiskUser,

      // self defined error
      101 => Error::InvalidJsonFromWechat,
      102 => Error::NetworkToWechatErr,
      103 => Error::UnionIdNotFound,
      104 => Error::TokenExpired,
      105 => Error::OpenIdNotFound,
      106 => Error::DatabaseErr,
      107 => Error::InvalidJsonRequest,
      _ => Error::UnknownErr,
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", Into::<String>::into(*self))
  }
}

impl std::error::Error for Error {}
