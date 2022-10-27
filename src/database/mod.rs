use crate::types::{SignUpInfo, LogInInfo, AccessToken};

use crypto::digest::Digest;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Acquire, Connection, MySql, MySqlConnection, Pool, query};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub mod wechat_op;

// database operation error definitions
#[derive(Copy, Clone, Debug)]
pub enum Error {}

#[derive(Debug)]
pub enum SignUpErr {
  UserExist,
  OtherErr,
}

impl Into<String> for SignUpErr {
  fn into(self) -> String {
    match self {
      SignUpErr::UserExist => "user not exist".into(),
      SignUpErr::OtherErr => "unknown error".into(),
    }
  }
}

#[derive(Debug)]
pub enum LogInErr {
  UserNotExist,
  PasswdNotMatch,
  OtherErr,
  // TODO: RecapchaErr,
}

impl Into<String> for LogInErr {
  fn into(self) -> String {
    match self {
      LogInErr::UserNotExist => "user not exist".into(),
      LogInErr::PasswdNotMatch => "password not match".into(),
      LogInErr::OtherErr => "unknown error".into(),
    }
  }
}

#[derive(Debug)]
pub struct ProspectSqlPool {
  pool: Pool<MySql>,
  rng: StdRng,
}

impl ProspectSqlPool {
  pub async fn new(user: String, pass: String, addr: String, database: String, max: u32) -> Result<ProspectSqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
      .max_connections(max)
      .connect(&format!("mysql://{}:{}@{}/{}", user, pass, addr, database)).await?;
    Ok(ProspectSqlPool {
      pool,
      rng: StdRng::from_entropy(),
    })
  }

  /// initialize necessary databases and tables backend needed.
  pub async fn init(user: String, pass: String, addr: String) -> Result<(), sqlx::Error> {
    let mut conn = MySqlConnection::connect(&format!("mysql://{}:{}@{}", user, pass, addr)).await?;
    let mut tx = sqlx::Connection::begin(&mut conn).await?;
    query("create database if not exists Prospect").execute(&mut tx).await?;
    query("create database if not exists UniUserMap").execute(&mut tx).await?;
    query("create table if not exists Prospect.tokenMap (\
           open_id varchar(255) not null ,\
           access_token blob(256) not null ,\
           expired_time timestamp not null ,\
           primary key (open_id)\
           )")
      .execute(&mut tx).await?;
    query("create table if not exists Prospect.Open2Union (\
           open_id varchar(255) not null ,\
           union_id varchar(255) not null ,\
           primary key (open_id)\
           );")
      .execute(&mut tx).await?;
    tx.commit();
    Ok(())
  }

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
        if cur_hash.as_bytes() != database_info.3.as_slice() {
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
