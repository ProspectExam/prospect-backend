use std::convert::Infallible;

use crate::types::*;

use log::{info, warn};

use super::types::*;
use super::common::*;
use super::types::AccessToken;

/// handler for /send_code
pub async fn send_code_handler(info: CodeInfo, mut ctx: Context) -> Result<impl warp::Reply, Infallible> {
  info!("a request with info: {:?}", info);
  let reply = if info.access_token.is_empty() {
    match code2session(&ctx.options.wx_appid, &ctx.options.wx_appsecret, &info.code).await {
      Ok(j) => {
        info!("json {:?} from wechat server parsed successfully", j);
        info!("require code2Session ok for code: {}", info.code);
        match j.errcode {
          Some(0) | None => if j.openid.is_some() {
            ctx.session = Some(j.clone());
            let open_id = j.openid.unwrap();
            let token = AccessToken::new(&open_id);
            info!("get json from wechat server with open_id {} and no error", open_id);
            match ctx.pool.wechat_record_token(token.clone(), ctx.clone()).await {
              Ok(()) => {
                info!("record access token {:?} for {} ok", token, open_id);
                CodeResult::new(Ok((open_id, token)))
              }
              Err(_) => {
                warn!("record token failed for {}", open_id);
                CodeResult::new(Err(Error::DatabaseErr))
              }
            }
          } else {
            info!("no open_id found in json from wechat server");
            CodeResult::new(Err(Error::OpenIdNotFound))
          }
          Some(err_code) => CodeResult::new(Err(err_code.into()))
        }
      }
      Err(e) => {
        warn!("request for code2Session failed for code {} {:?}", info.code, e);
        CodeResult::new(Err(e))
      }
    }
  } else {
    info!("get info with access token from miniprogram, querying database cache...");
    match ctx.pool.valid_token_and_update(
      AccessToken::from(info.access_token.clone()),
      &info.open_id,
    ).await {
      Ok(token) => {
        info!("access token {} cache HIT!", info.access_token);
        CodeResult::new(Ok((info.open_id, token)))
      }
      Err(_) => {
        info!("access token {} cache expired", info.access_token);
        CodeResult::new(Err(Error::TokenExpired))
      }
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
  info!("a request with info: {:?}", info);
  let reply = match ctx.pool.is_valid_access_token(&info.open_id, info.access_token.into()).await {
    Ok(true) => {
      info!("access token valid, subscribe {}.{} for {}",
        info.school_code,
        info.department_code,
        info.open_id);
      match ctx.pool.wechat_subscribe(&info.open_id, ctx.clone()).await {
        Ok(()) => SubscribeResult::new(Ok(())),
        Err(_) => SubscribeResult::new(Err(Error::DatabaseErr)),
      }
    }
    Ok(false) => {
      info!("access token expired from {}", info.open_id);
      SubscribeResult::new(Err(Error::TokenExpired))
    }
    Err(_) => {
      warn!("querying token failed caused by database");
      SubscribeResult::new(Err(Error::DatabaseErr))
    }
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

pub async fn get_university_info_handler(ctx: Context) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::reply())
}
