/// wechat sql api definitions

use chrono::{prelude::*, Duration};

use crate::wechat::types::{AccessToken, Context};
use super::ProspectSqlPool;

impl ProspectSqlPool {
  pub async fn wechat_record_token(&self, token: AccessToken, ctx: Context) -> Result<(), sqlx::Error> {
    self.record_token_helper(
      token,
      &ctx.session.as_ref().unwrap().openid.as_ref().unwrap(),
      &ctx.session.as_ref().unwrap().unionid.as_ref().unwrap(),
    ).await
  }

  async fn record_token_helper(&self, token: AccessToken, open_id: &str, union_id: &str) -> Result<(), sqlx::Error> {
    let mut tx = sqlx::Pool::begin(&self.pool).await?;
    let sql =
      "INSERT INTO Prospect.Open2Union (open_id, union_id) \
       VALUES (?, ?) ON DUPLICATE KEY UPDATE union_id = ?";
    sqlx::query(sql)
      .bind(open_id)
      .bind(union_id)
      .bind(union_id)
      .execute(&mut tx).await?;
    let sql =
      "INSERT INTO Prospect.tokenMap (open_id, access_token, expired_time) \
       VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE access_token = ?, expired_time = ?";
    sqlx::query(sql)
      .bind(open_id)
      .bind(&token.token)
      .bind(&token.expired)
      .bind(&token.token)
      .bind(&token.expired)
      .execute(&mut tx).await?;
    tx.commit().await?;
    Ok(())
  }

  /// check token passed in if expired, then update time if not.
  pub async fn valid_token_and_update(&self, token: AccessToken, open_id: &str) -> Result<AccessToken, sqlx::Error> {
    let mut tx = sqlx::Pool::begin(&self.pool).await?;

    // query if there is a valid access token
    let sql =
      "SELECT Prospect.tokenMap.open_id AS open_id, Prospect.Open2Union.union_id AS union_id \
       FROM Prospect.tokenMap, Prospect.Open2Union \
       WHERE Prospect.tokenMap.open_id = ? \
       AND Prospect.tokenMap.access_token = ? \
       AND Prospect.tokenMap.expired_time > ? \
       AND Prospect.Open2Union.open_id = Prospect.tokenMap.open_id";
    let rows: (String, String) = sqlx::query_as(sql)
      .bind(open_id)
      .bind(&token.token)
      .bind(Utc::now())
      .fetch_one(&mut tx).await?;

    // update if there is a valid access token
    let new_token = AccessToken::new(&rows.0, &rows.1);
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
