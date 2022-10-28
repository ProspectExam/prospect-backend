use std::borrow::BorrowMut;
use std::convert::Infallible;

use crate::types::*;

use log::{info, warn};

use super::types::*;
use super::common::*;
use super::types::AccessToken;

/// handler for /send_code
pub async fn send_code_handler(info: CodeInfo, mut ctx: Context) -> Result<impl warp::Reply, Infallible> {
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
        match j.errcode {
          Some(0) | None => if j.openid.is_some() && j.unionid.is_some() {
            ctx.session = Some(j.clone());
            let open_id = j.openid.unwrap();
            let union_id = j.unionid.unwrap();
            let token = AccessToken::new(&open_id, &union_id);
            match ctx.pool.wechat_record_token(token.clone(), ctx.clone()).await {
              Ok(()) => {
                info!("record token for {} ok", open_id);
                CodeResult::new(Ok(token.into()))
              }
              Err(_) => {
                warn!("record token failed for {}", open_id);
                CodeResult::new(Err(Error::DatabaseErr))
              }
            }
          } else if j.openid.is_none() {
            CodeResult::new(Err(Error::OpenIdNotFound))
          } else {
            CodeResult::new(Err(Error::UnionIdNotFound))
          },
          Some(err_code) => CodeResult::new(Err(err_code.into()))
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

// handler for waterfall
pub async fn waterfall_handler(ctx: Context) -> Result<impl warp::Reply, Infallible> {
  // TODO: todo!()
  Ok(warp::reply::reply())
}

// handler for subscribe
pub async fn subscribe_handler(info: SubscribeInfo, ctx: Context) -> Result<impl warp::Reply, Infallible> {
  // TODO: todo!()
  Ok(warp::reply::reply())
}
