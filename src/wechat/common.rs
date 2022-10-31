use chrono::{Duration, Utc};
use reqwest::get;

use serde::{Deserialize, Serialize};

use super::types::*;

/// Get json from specified URL.
pub async fn get_json_from_url<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Error> {
  match get(url).await {
    Ok(r) => {
      match r.json::<T>().await {
        Ok(j) => {
          Ok(j)
        }
        Err(_) => {
          Err(Error::InvalidJson)
        }
      }
    }
    Err(_) => {
      Err(Error::NetworkToWechatErr)
    }
  }
}

/// Make post request for json with specified post data.
pub async fn post_json_to_url<T, U>(url: &str, post_data: U) -> Result<T, Error>
  where U: Serialize, T: for<'de> Deserialize<'de> {
  // TODO:
  Err(Error::InvalidJson)
}

pub(crate) async fn code2session(app_id: &str, app_secret: &str, code: &str) -> Result<Code2SessionResponse, Error> {
  // construct auth.code2Session request URL
  let code2session = format!(
    "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
    app_id,
    app_secret,
    code,
  );
  get_json_from_url::<Code2SessionResponse>(&code2session).await
}

/// Get access token for miniprogram from wechat server.
pub(crate) async fn get_access_token(ctx: Context) -> Result<String, Error> {
  // return cached access token if it's not expired,
  // or request from wechat server.
  {
    // TODO: handle err cause by unwrap
    let ctx_lock = ctx.global_field.lock().unwrap();
    if &Utc::now() < ctx_lock.expired_time.as_ref().unwrap() {
      return Ok(ctx_lock.access_token.as_ref().unwrap().clone());
    }
  }
  let req = format!(
    "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
    ctx.options.wx_appid,
    ctx.options.wx_appsecret,
  );
  let r = get_json_from_url::<GetAccessTokenResponse>(&req).await?;
  let mut ctx_lock = ctx.global_field.lock().unwrap();
  ctx_lock.access_token = r.access_token.clone();
  ctx_lock.expired_time = Some(Utc::now() + Duration::seconds(r.expires_in.unwrap() as i64));
  Ok(r.access_token.unwrap())
}
