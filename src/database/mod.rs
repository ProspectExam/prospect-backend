use std::borrow::BorrowMut;
use std::sync::Arc;
use crate::types::{SignUpInfo, LogInInfo, AccessToken};

use crypto::digest::Digest;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Connection, MySql, MySqlConnection, Pool, query};

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

#[derive(Clone, Debug)]
pub struct ProspectSqlPool {
  pool: Pool<MySql>,
  rng: Arc<tokio::sync::Mutex<StdRng>>,
}

// public operation for ProspectSqlPool
impl ProspectSqlPool {
  pub async fn new(user: String, pass: String, addr: String, database: String, max: u32) -> Result<ProspectSqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
      .max_connections(max)
      .connect(&format!("mysql://{}:{}@{}/{}", user, pass, addr, database)).await?;
    Ok(ProspectSqlPool {
      pool,
      rng: Arc::new(tokio::sync::Mutex::new(StdRng::from_entropy())),
    })
  }

  /// initialize necessary databases and tables backend needed.
  pub async fn init(user: String, pass: String, addr: String) -> Result<(), sqlx::Error> {
    let mut conn = MySqlConnection::connect(&format!("mysql://{}:{}@{}", user, pass, addr)).await?;
    let mut tx = sqlx::Connection::begin(&mut conn).await?;
    // create Prospect main database
    query("CREATE DATABASE IF NOT EXISTS Prospect").execute(&mut tx).await?;
    // create database for universities and departments store
    query("CREATE DATABASE IF NOT EXISTS UniUserMap").execute(&mut tx).await?;
    // open_id --- access_token --- expired_time map
    query("CREATE TABLE IF NOT EXISTS Prospect.tokenMap (\
           open_id VARCHAR(255) NOT NULL ,\
           access_token BLOB(256) NOT NULL ,\
           expired_time TIMESTAMP NOT NULL ,\
           PRIMARY KEY (open_id)\
           )")
      .execute(&mut tx).await?;
    // university_id --- university_name map
    query("CREATE TABLE IF NOT EXISTS UniUserMap.university (\
           id INT UNSIGNED NOT NULL AUTO_INCREMENT ,\
           uni_name VARCHAR(128) NOT NULL ,\
           name VARCHAR(1024) NOT NULL ,\
           PRIMARY KEY (id) ,\
           UNIQUE KEY (uni_name) \
           ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci")
      .execute(&mut tx).await?;
    tx.commit().await?;
    Ok(())
  }

  pub async fn add_university(&self, uni_name: &str, name: &str) -> Result<(), sqlx::Error> {
    let mut tx = self.pool.begin().await?;
    let uni_name = format!("{}_university", uni_name);
    query("INSERT INTO UniUserMap.university (uni_name, name) VALUES (?, ?)")
      .bind(&uni_name)
      .bind(name)
      .execute(&mut tx).await?;
    query(&format!(
      "CREATE TABLE UniUserMap.{} (\
       id INT UNSIGNED NOT NULL AUTO_INCREMENT ,\
       uni_name VARCHAR(128) NOT NULL ,\
       department_name VARCHAR(1024) NOT NULL ,\
       PRIMARY KEY (id) ,\
       UNIQUE KEY (uni_name) \
       ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci", uni_name
    )).execute(&mut tx).await?;
    tx.commit().await?;
    Ok(())
  }

  pub async fn add_department(&self, university_id: u32, uni_name: &str, name: &str) -> Result<(), sqlx::Error> {
    let mut tx = self.pool.begin().await?;
    let university_uni_name: (String, ) =
      sqlx::query_as("SELECT uni_name FROM UniUserMap.university WHERE id = ?")
        .bind(university_id)
        .fetch_one(&mut tx).await?;
    let sql = format!("INSERT INTO UniUserMap.{} (uni_name, department_name) VALUES (?, ?)", university_uni_name.0);
    let uni_name = format!("{}_{}_depart", university_uni_name.0, uni_name);
    query(&sql)
      .bind(&uni_name)
      .bind(name)
      .execute(&mut tx).await?;
    let sql = format!(
      "CREATE TABLE UniUserMap.{} (\
       open_id VARCHAR(255) NOT NULL ,\
       PRIMARY KEY (open_id))",
      uni_name
    );
    query(&sql).execute(&mut tx).await?;
    tx.commit().await?;
    Ok(())
  }

  pub async fn subscribe_user(&self, open_id: &str, university_id: u32, department_id: u32) -> Result<(), sqlx::Error> {
    let mut tx = self.pool.begin().await?;
    // locate to department table
    let university_uni_name: (String, ) =
      sqlx::query_as("SELECT uni_name FROM UniUserMap.university WHERE id = ?")
        .bind(university_id)
        .fetch_one(&mut tx).await?;
    let department_uni_name: (String, ) =
      sqlx::query_as(&format!("SELECT uni_name FROM UniUserMap.{} WHERE id = ?", university_uni_name.0))
        .bind(department_id)
        .fetch_one(&mut tx).await?;
    // insert into department table
    let sql = format!("INSERT INTO UniUserMap.{} (open_id) VALUES (?)", department_uni_name.0);
    query(&sql)
      .bind(open_id)
      .execute(&mut tx).await?;
    tx.commit().await?;
    Ok(())
  }

  pub async fn get_users(&self, university_id: u32, department_id: u32) -> Result<Vec<String>, sqlx::Error> {
    let sql = "SELECT uni_name from UniUserMap.university WHERE id = ?";
    let university_uni_name: (String, ) = sqlx::query_as(sql)
      .bind(university_id)
      .fetch_one(&self.pool).await?;
    let sql = format!("SELECT uni_name from UniUserMap.{} WHERE id = ?", university_uni_name.0);
    let department_uni_name: (String, ) = sqlx::query_as(&sql)
      .bind(department_id)
      .fetch_one(&self.pool).await?;
    let sql = format!("SELECT open_id from UniUserMap.{}", department_uni_name.0);
    let rows: Vec<(String, )> = sqlx::query_as(&sql)
      .fetch_all(&self.pool).await?;
    Ok(rows.into_iter().map(|(open_id, )| open_id).collect())
  }

  pub async fn sign_up(&self, info: SignUpInfo) -> Result<(), SignUpErr> {
    let r: Result<(u32, ), _> = sqlx::query_as("select user_id from UserAuth where username = ?")
      .bind(&info.username)
      .fetch_one(&self.pool)
      .await;
    match r {
      Ok(_) => {
        Err(SignUpErr::UserExist)
      }
      Err(sqlx::Error::RowNotFound) => {
        let (salt, pass_and_salt_hash) = self.KDF(&info.password).await;
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

  pub async fn log_in(&self, info: LogInInfo) -> Result<(u32, AccessToken), LogInErr> {
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

  async fn KDF(&self, passwd: &str) -> ([u8; 8], String) {
    let salt = self.rng.lock().await.borrow_mut().gen::<[u8; 8]>();
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
