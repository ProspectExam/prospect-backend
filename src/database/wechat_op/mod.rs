/// wechat sql api definitions
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
}
