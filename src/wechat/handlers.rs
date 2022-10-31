use std::convert::Infallible;

use crate::types::*;

use log::{info, warn};

use super::types::*;
use super::common::*;
use super::types::AccessToken;

/// handler for /send_code
pub async fn send_code_handler(info: CodeInfo, mut ctx: Context) -> Result<impl warp::Reply, Infallible> {
  let reply = if info.access_token.is_empty() {
    match code2session(&ctx.options.wx_appid, &ctx.options.wx_appsecret, &info.code).await {
      Ok(j) => {
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
                CodeResult::new(Ok((open_id, token)))
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
    match ctx.pool.valid_token_and_update(
      AccessToken::from(info.access_token),
      &info.open_id,
    ).await {
      Ok(token) => CodeResult::new(Ok((info.open_id, token))),
      Err(_) => CodeResult::new(Err(Error::TokenExpired)),
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
pub async fn subscribe_handler(info: SubscribeInfo, mut ctx: Context) -> Result<impl warp::Reply, Infallible> {
  let reply = match ctx.pool.is_valid_access_token(&info.open_id, info.access_token.into()).await {
    Ok(true) => {
      match ctx.pool.wechat_subscribe(&info.open_id, ctx.clone()).await {
        Ok(()) => SubscribeResult::new(Ok(())),
        Err(_) => SubscribeResult::new(Err(Error::DatabaseErr)),
      }
    }
    Ok(false) => { SubscribeResult::new(Err(Error::TokenExpired)) }
    Err(_) => { SubscribeResult::new(Err(Error::DatabaseErr)) }
  };
  Ok(warp::reply::json(&reply))
}

async fn notify_subscription() {
  // TODO:
  // ctx.session = Some(Code2SessionResponse {
  //   openid: Some(info.open_id),
  //   session_key: None,
  //   unionid: None,
  //   errcode: None,
  //   errmsg: None
  // });
  // let wechat_access_token = get_access_token(ctx.clone());
  unimplemented!()
}
