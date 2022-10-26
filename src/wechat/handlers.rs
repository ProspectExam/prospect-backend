use std::convert::Infallible;
use std::future::Future;

use crate::types::*;

use log::info;
use reqwest::get;

use super::types::*;
use super::common::*;

pub async fn send_code_handler(info: CodeInfo, ctx: Context) -> Result<impl warp::Reply, Infallible> {
  let reply = if info.access_token.is_empty() {
    // construct auth.code2Session request URL
    let code2session = format!(
      "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
      &ctx.options.wx_appid,
      &ctx.options.wx_appsecret,
      &info.code
    );
    match get_json::<Code2SessionResponse>(&code2session).await {
      Ok(j) => {
        info!("{:?} from wechat server", j);
        // TODO: register cache, generate token then ret
        CodeResult {
          success: true,
          message: "ok".to_string(),
          access_token: "1234567890".to_string(),
        }
      }
      Err(e) => {
        CodeResult {
          success: false,
          message: e.into(),
          access_token: "".to_string(),
        }
      }
    }
  } else {
    // TODO: check cache
    CodeResult {
      success: false,
      message: "".to_string(),
      access_token: "".to_string(),
    }
  };
  Ok(warp::reply::json(&reply))
}

pub async fn waterfall_handler(ctx: Context) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::reply())
}
