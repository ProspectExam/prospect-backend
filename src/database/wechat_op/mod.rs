use crate::wechat::types::{AccessToken, Context};
/// wechat sql api definitions
use super::ProspectSqlPool;

impl ProspectSqlPool {
  pub async fn wechat_record_token(&self, token: AccessToken, ctx: Context) -> Result<(), sqlx::Error> {
    self.record_token_helper(token, &ctx.session.unwrap().openid.unwrap()).await
  }

  async fn record_token_helper(&self, token: AccessToken, open_id: &str) -> Result<(), sqlx::Error> {
    let sql = "insert into Prospect.tokenMap (open_id, access_token, expired_time) VALUES (?, ?, ?)";
    let _ = sqlx::query(sql)
      .bind(open_id)
      .bind(token.token)
      .bind(token.expired)
      .execute(&self.pool).await?;
    Ok(())
  }
}
