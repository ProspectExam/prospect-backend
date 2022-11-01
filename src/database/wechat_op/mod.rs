/// wechat sql api definitions

use chrono::{prelude::*, Duration};

use crate::wechat::types::{AccessToken, Context};
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
    let r: (u32, ) = sqlx::query_as(sql)
      .bind(open_id)
      .bind(&token.token)
      .bind(Utc::now())
      .fetch_one(&self.pool).await?;
    match r {
      (0, ) => Ok(false),
      _ => Ok(true),
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
  pub async fn wechat_subscribe(&self, open_id: &str, ctx: Context) -> Result<(), sqlx::Error> {
    // TODO:
    Ok(())
  }
}
