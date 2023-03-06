use std::convert::Infallible;

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
  let mut items = Vec::new();
  // read dirs
  for file in std::fs::read_dir("./assets/waterfall").unwrap() {
    if file.as_ref().map_or(false, |f| f.file_type().map_or(false, |t| t.is_file())) {
      let filename = file.unwrap().file_name().into_string().unwrap();
      items.push(WaterFallItem::new("".into(), filename.clone(), "waterfall/".to_string() + &filename));
    }
  }
  let reply = WaterFall::new(Ok(items));
  Ok(warp::reply::json(&reply))
}

// handler for subscribe
pub async fn subscribe_handler(info: SubscribeInfo, ctx: Context) -> Result<impl warp::Reply, Infallible> {
  info!("a request with info: {:?}", info);
  let reply = match ctx.pool.is_valid_access_token(&info.open_id, info.access_token.clone().into()).await {
    Ok(true) => {
      info!("access token valid, subscribe for {}", &info.open_id);
      match ctx.pool.wechat_subscribe(info, ctx.clone()).await {
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

pub async fn get_user_subscribe_handler(info: GetSubscribeInfo, ctx: Context) -> Result<impl warp::Reply, Infallible> {
  info!("a request with info: {:?}", info);
  let reply = match ctx.pool.is_valid_access_token(&info.open_id, info.access_token.clone().into()).await {
    Ok(true) => {
      info!("access token valid, get subscribe for {}", &info.open_id);
      match ctx.pool.wechat_get_subscribe(info, ctx.clone()).await {
        Ok(sub) => GetSubscribeResult::new(Ok(sub)),
        Err(_) => GetSubscribeResult::new(Err(Error::DatabaseErr)),
      }
    }
    Ok(false) => {
      info!("access token expired from {}", info.open_id);
      GetSubscribeResult::new(Err(Error::TokenExpired))
    }
    Err(_) => {
      warn!("querying token failed caused by database");
      GetSubscribeResult::new(Err(Error::DatabaseErr))
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

pub async fn get_university_handler(ctx: Context) -> Result<impl warp::Reply, Infallible> {
  let reply = {
    match ctx.pool.wechat_get_university().await {
      Ok(hashmap) => UniversityResult::new(Ok(hashmap)),
      Err(sqlx::Error::RowNotFound) => UniversityResult::new(Err(Error::InvalidJsonRequest)),
      Err(_) => UniversityResult::new(Err(Error::DatabaseErr)),
    }
  };
  Ok(warp::reply::json(&reply))
}

pub async fn get_department_handler(info: GetDepartmentInfo, ctx: Context) -> Result<impl warp::Reply, Infallible> {
  let reply = {
    match ctx.pool.wechat_get_department(info.university_code).await {
      Ok(hashmap) => DepartmentResult::new(Ok(hashmap)),
      Err(sqlx::Error::RowNotFound) => DepartmentResult::new(Err(Error::InvalidJsonRequest)),
      Err(_) => DepartmentResult::new(Err(Error::DatabaseErr)),
    }
  };
  Ok(warp::reply::json(&reply))
}
