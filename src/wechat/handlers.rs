use std::convert::Infallible;
use std::future::Future;

use crate::types::*;

use log::{info, warn};
use reqwest::get;

use super::types::*;
use super::common::*;
use super::types::AccessToken;

/// handler for /send_code
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
      Ok(mut j) => {
        info!("{:?} from wechat server", j);
        info!("require code2Session ok for {}", info.code);
        match j.unionid.take() {
          None => {
            warn!("no unionid in code2Session response");
            match j.errcode.take() {
              // no union_id in response, means wechat not verified
              None => CodeResult::new(Err(Error::UnionIdNotFound)),
              Some(err_id) => CodeResult::new(Err(err_id.into())),
            }
          }
          Some(union_id) => {
            // union_id got
            // TODO: generate access token to memo then return
            warn!("unionid in code2Session response: {}", union_id);
            CodeResult::new(Ok(AccessToken::new())) }
        }
      }
      Err(e) => {
        info!("require code2Session failed for {}", info.code);
        CodeResult::new(Err(e))
      }
    }
  } else {
    // TODO: check cache
    if AccessToken::from(info.access_token).is_valid() {
      CodeResult::new(Ok(AccessToken::empty()))
    } else {
      CodeResult::new(Err(Error::TokenExpired))
    }
  };
  Ok(warp::reply::json(&reply))
}

// handler for
pub async fn waterfall_handler(ctx: Context) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::reply())
}
