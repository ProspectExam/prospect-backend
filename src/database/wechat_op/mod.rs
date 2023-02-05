/// wechat sql api definitions

use std::collections::{HashMap, VecDeque};

use chrono::prelude::*;
use log::{info, warn};

use crate::wechat::common::get_access_token;
use crate::wechat::to_wechat_types::{SendMessage, SendMessageResult, SubscribeTemplate};
use crate::wechat::types::{AccessToken, Context, Error, GetSubscribeInfo, SubscribeDetail, SubscribeInfo};

use super::ProspectSqlPool;

// impl for public wechat operation
impl ProspectSqlPool {
  pub async fn is_valid_access_token(&self, open_id: &str, token: AccessToken) -> Result<bool, sqlx::Error> {
    let sql =
      "SELECT COUNT(*) \
       FROM Prospect.tokenMap \
       WHERE open_id = ? \
       AND Prospect.tokenMap.access_token = ? \
       AND Prospect.tokenMap.expired_time > ?";
    let r: (i32, ) = sqlx::query_as(sql)
      .bind(open_id)
      .bind(&token.token)
      .bind(Utc::now())
      .fetch_one(&self.pool).await?;
    match r {
      (0, ) => Ok(false),
      _ => Ok(true),
    }
  }

  pub async fn wechat_notify(&self, university_id: u32, department_id: u32, ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
    let users = self.get_users(university_id, department_id).await?;
    let mut users = users.into_iter().map(|u| (u, 5)).collect::<VecDeque<_>>();
    let access_token = get_access_token(ctx.clone()).await?;
    let url = format!("https://api.weixin.qq.com/cgi-bin/message/subscribe/send?access_token={}", access_token);
    warn!("request to wechat server for notification");
    // get university and department name
    let university = self.get_university_name(university_id).await?;
    let department = self.get_department_name(university_id, department_id).await?;
    // prepare POST for wechat server
    // let data = TestSendMessageTemplate::new("tsinghua".into(), "2020-01-01".into());
    let data =
      SubscribeTemplate::new(
        university,
        department,
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      );
    let mut post_struct = SendMessage {
      template_id: "TMFuXpbbjg21tEN1c4D_kHGtsNuRccqo7ft3aBC2J6s".to_string(),
      touser: "".to_string(),
      data: data,
      miniprogram_state: "developer".to_string(),
      lang: "zh_CN".to_string(),
    };
    // send POST request to wechat server
    // try at most 5 times for each user
    let client = reqwest::Client::new();
    let mut failed_users = Vec::<(String, Error)>::new();
    while let Some(mut user) = users.pop_front() {
      let (ref user_id, ref mut times) = user;
      post_struct.touser = user_id.clone();
      *times -= 1;
      let res = client.post(&url)
        .json(&post_struct)
        .send()
        .await?;
      if res.status().is_success() {
        match res.json::<SendMessageResult>().await {
          Ok(obj) => if obj.errcode != 0 {
            failed_users.push((user_id.clone(), obj.errcode.into()));
          } else {
            info!("send message to user {} successfully", user_id);
            // remove user from subscribe database
            // construct SubscribeInfo
            let info = SubscribeInfo {
              open_id: user_id.clone(),
              access_token: "".into(),
              info: vec![SubscribeDetail {
                school_code: university_id,
                department_code: department_id,
                oper: 1,
              }],
            };
            self.subscribe_user(info).await?;
          },
          Err(_) => failed_users.push((user_id.clone(), Error::InvalidJsonFromWechat)),
        }
      } else {
        if *times > 0 {
          users.push_back(user);
        } else {
          failed_users.push((user_id.clone(), Error::NetworkToWechatErr));
        }
      }
    }
    if !failed_users.is_empty() {
      Err(format!("failed to send message to users: {:?}", failed_users).into())
    } else {
      Ok(())
    }
  }
}

// impl for send_code
impl ProspectSqlPool {
  pub async fn wechat_record_token(&self, token: AccessToken, ctx: Context) -> Result<(), sqlx::Error> {
    self.record_token_helper(
      token,
      &ctx.session.as_ref().unwrap().openid.as_ref().unwrap(),
    ).await
  }

  async fn record_token_helper(&self, token: AccessToken, open_id: &str) -> Result<(), sqlx::Error> {
    let sql =
      "INSERT INTO Prospect.tokenMap (open_id, access_token, expired_time) \
       VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE access_token = ?, expired_time = ?";
    sqlx::query(sql)
      .bind(open_id)
      .bind(&token.token)
      .bind(&token.expired)
      .bind(&token.token)
      .bind(&token.expired)
      .execute(&self.pool).await?;
    Ok(())
  }

  /// check token passed in if expired, then update time if not.
  pub async fn valid_token_and_update(&self, token: AccessToken, open_id: &str) -> Result<AccessToken, sqlx::Error> {
    let mut tx = sqlx::Pool::begin(&self.pool).await?;

    // query if there is a valid access token
    let sql =
      "SELECT open_id FROM Prospect.tokenMap WHERE open_id = ? AND access_token = ? AND expired_time > ?";
    let rows: (String, ) = sqlx::query_as(sql)
      .bind(open_id)
      .bind(&token.token)
      .bind(Utc::now())
      .fetch_one(&mut tx).await?;

    // update if there is a valid access token
    let new_token = AccessToken::new(&rows.0);
    let sql =
      "UPDATE Prospect.tokenMap \
       SET access_token = ?, expired_time = ? \
       WHERE open_id = ?";
    sqlx::query(sql)
      .bind(&new_token.token)
      .bind(&new_token.expired)
      .bind(&rows.0)
      .execute(&mut tx).await?;
    tx.commit().await?;

    Ok(new_token)
  }
}

// impl for subscribe
impl ProspectSqlPool {
  pub async fn wechat_subscribe(&self, info: SubscribeInfo, ctx: Context) -> Result<(), sqlx::Error> {
    self.subscribe_user(info).await?;

    Ok(())
  }

  pub async fn wechat_get_subscribe(&self, info: GetSubscribeInfo, ctx: Context) -> Result<HashMap<u32, Vec<u32>>, sqlx::Error> {
    let table_name = format!("UserSubMap.u{}", info.open_id);
    let sql =
      format!("SELECT university_id, department_id FROM {}", table_name);
    let rows: Vec<(u32, u32)> =
      sqlx::query_as(&sql)
        .fetch_all(&self.pool).await?;
    let map = rows
      .into_iter()
      .fold(HashMap::new(), |mut map, (uni_id, dep_id)| {
        map.entry(uni_id).or_insert_with(Vec::new).push(dep_id);
        map
      });
    Ok(map)
  }

  pub async fn wechat_get_university(&self) -> Result<HashMap<u32, String>, sqlx::Error> {
    let sql = "SELECT id, name FROM UniUserMap.university";
    let rows: Vec<(u32, String)> = sqlx::query_as(sql).fetch_all(&self.pool).await?;
    let map = rows
      .into_iter()
      .collect::<HashMap<_, _>>();
    Ok(map)
  }

  pub async fn wechat_get_department(&self, university_id: u32) -> Result<HashMap<u32, String>, sqlx::Error> {
    let sql = "SELECT uni_name FROM UniUserMap.university WHERE id = ?";
    let university_name: (String, ) = sqlx::query_as(sql)
      .bind(university_id)
      .fetch_one(&self.pool).await?;
    let sql = format!("SELECT id, department_name FROM UniUserMap.{}", university_name.0);
    let rows: Vec<(u32, String)> = sqlx::query_as(&sql).bind(university_id).fetch_all(&self.pool).await?;
    let map = rows
      .into_iter()
      .collect::<HashMap<_, _>>();
    Ok(map)
  }
}
