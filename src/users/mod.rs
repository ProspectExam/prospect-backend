use crate::types::{SignUpInfo, SignUpResult, LogInInfo, LogInResult, AccessToken};

use std::sync::Arc;
use crypto::digest::Digest;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub enum SignUpErr {
  UserExist,
  OtherErr,
}

#[derive(Debug)]
pub enum LogInErr {
  UserNotExist,
  PasswdNotMatch,
  OtherErr,
  // TODO: RecapchaErr,
}

#[derive(Debug)]
pub struct ProspectSqlPool {
  pool: Pool<MySql>,
  rng: StdRng,
}

pub async fn new_pool(user: String, pass: String, addr: String, max: u32) -> Result<ProspectSqlPool, sqlx::Error> {
  let pool = MySqlPoolOptions::new()
    .max_connections(max)
    .connect(&format!("mysql://{}:{}@{}/prospect", user, pass, addr)).await?;
  Ok(ProspectSqlPool {
    pool,
    rng: StdRng::from_entropy(),
  })
}

impl ProspectSqlPool {
  pub async fn sign_up(&mut self, info: SignUpInfo) -> Result<(), SignUpErr> {
    let r: Result<(u32, ), _> = sqlx::query_as("select user_id from UserAuth where username = ?")
      .bind(&info.username)
      .fetch_one(&self.pool)
      .await;
    match r {
      Ok(_) => {
        Err(SignUpErr::UserExist)
      }
      Err(sqlx::Error::RowNotFound) => {
        let (salt, pass_and_salt_hash) = self.KDF(&info.password);
        // insert a user message to database
        sqlx::query("insert into UserAuth (username, salt, hash) values (?, ?, ?)")
          .bind(&info.username)
          .bind(salt.as_slice())
          .bind(&pass_and_salt_hash)
          .execute(&self.pool)
          .await.map_err(|_| SignUpErr::OtherErr)?;
        Ok(())
      }
      _ => {
        Err(SignUpErr::OtherErr)
      }
    }
  }

  pub async fn log_in(&mut self, info: LogInInfo) -> Result<(u32, AccessToken), LogInErr> {
    let r: Result<(u32, String, Vec<u8>, Vec<u8>), _> =
      sqlx::query_as("select user_id, username, salt, hash from UserAuth where username = ?")
        .bind(&info.username)
        .fetch_one(&self.pool)
        .await;
    match r {
      Ok(database_info) => {
        let cur_hash = Self::KDF_with_salt(&database_info.2, &info.password);
        if cur_hash.as_bytes() != database_info.3.as_slice()   {
          Err(LogInErr::PasswdNotMatch)
        } else {
          Ok((database_info.0, AccessToken))
        }
      }
      Err(sqlx::Error::RowNotFound) => {
        // user not found
        Err(LogInErr::UserNotExist)
      }
      _ => {
        // unknown error
        Err(LogInErr::OtherErr)
      }
    }
  }

  fn KDF(&mut self, passwd: &str) -> ([u8; 8], String) {
    let salt = self.rng.gen::<[u8; 8]>();
    (salt, Self::KDF_with_salt(&salt, passwd))
  }

  fn KDF_with_salt(salt: &[u8], passwd: &str) -> String {
    let mut hasher = crypto::sha3::Sha3::sha3_256();
    let b = passwd.as_bytes()
      .iter()
      .chain(
        salt.iter()
      )
      .map(|x| *x)
      .collect::<Vec<u8>>();
    hasher.input(b.as_slice());
    hasher.result_str()
  }
}
