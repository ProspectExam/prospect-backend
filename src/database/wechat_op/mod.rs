/// wechat sql api definitions

use std::collections::{HashMap, VecDeque};

use chrono::{prelude::*, Duration};
use warp::query;

use crate::wechat::common::get_access_token;
use crate::wechat::to_wechat_types::{SendMessage, SendMessageResult, SubscribeTemplate, TestSendMessageTemplate};
use crate::wechat::types::{AccessToken, Context, Error, SubscribeInfo};

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
    // prepare POST for wechat server
    // let data = SubscribeTemplate {};
    let data = TestSendMessageTemplate::new("test title opening class".into(), "2020-01-01".into());
    let mut post_struct = SendMessage {
      template_id: "GtfweX744wEk1OFMOLivAM15GRYkL6x1Dsgkwcjjd6M".to_string(),
      to_user: "".to_string(),
      data: serde_json::to_string(&data)?,
      miniprogram_state: "developer".to_string(),
      lang: "zh_CN".to_string(),
    };
    // send POST request to wechat server
    // try at most 5 times for each user
    let client = reqwest::Client::new();
    let mut failed_users = Vec::<(String, Error)>::new();
    while let Some(mut user) = users.pop_front() {
      let (ref user_id, ref mut times) = user;
      post_struct.to_user = user_id.clone();
      *times -= 1;
      let res = client.post(&url)
        .json(&post_struct)
        .send()
        .await?;
      if res.status().is_success() {
        match res.json::<SendMessageResult>().await {
          Ok(obj) => if obj.errcode != 0 {
            failed_users.push((user_id.clone(), obj.errcode.into()));
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
    // let mut tx = sqlx::Pool::begin(&self.pool).await?;
    // // get university uni_id
    // let university_uni: (String, ) =
    //   sqlx::query_as("SELECT uni_name FROM Prospect.university WHERE id = ?")
    //     .bind(&info.school_code)
    //     .fetch_one(&mut tx).await?;
    // // get department uni_id
    // let sql = format!(
    //   "SELECT uni_name FROM Prospect.department WHERE uni_name = '{}' AND id = ?",
    //   university_uni.0
    // );
    // let department_uni: (String, ) =
    //   sqlx::query_as(&sql)
    //     .bind(&info.department_code)
    //     .fetch_one(&mut tx).await?;
    // let sql = format!(
    //   "INSERT INTO Prospect.{} (open_id) VALUES (?)",
    //   department_uni.0
    // );
    // sqlx::query(&sql)
    //   .bind(&ctx.session.as_ref().unwrap().openid.as_ref().unwrap())
    //   .bind(&university_uni.0)
    //   .bind(&department_uni.0)
    //   .bind(Utc::now())
    //   .execute(&mut tx).await?;
    // tx.commit().await?;
    self.subscribe_user(&info.open_id, info.school_code, info.department_code).await?;

    Ok(())
  }

  pub async fn wechat_get_university(&self) -> Result<HashMap<String, u32>, sqlx::Error> {
    let sql = "SELECT id, name FROM UniUserMap.university";
    let rows: Vec<(u32, String)> = sqlx::query_as(sql).fetch_all(&self.pool).await?;
    let mut map = rows
      .into_iter()
      .map(|(id, name)| (name, id))
      .collect::<HashMap<String, u32>>();
    Ok(map)
  }

  pub async fn wechat_get_department(&self, university_id: u32) -> Result<HashMap<String, u32>, sqlx::Error> {
    let sql = "SELECT uni_name FROM UniUserMap.university WHERE id = ?";
    let university_name: (String, ) = sqlx::query_as(sql)
      .bind(university_id)
      .fetch_one(&self.pool).await?;
    let sql = format!("SELECT id, department_name FROM UniUserMap.{}", university_name.0);
    let rows: Vec<(u32, String)> = sqlx::query_as(&sql).bind(university_id).fetch_all(&self.pool).await?;
    let mut map = rows
      .into_iter()
      .map(|(id, name)| (name, id))
      .collect::<HashMap<String, u32>>();
    Ok(map)
  }
}
