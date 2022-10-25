use crate::types::{SignUpInfo, SignUpResult};

use std::sync::Arc;
use crypto::digest::Digest;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use tokio::io::AsyncReadExt;

pub enum SignUpErr {
  UserExist,
}

pub enum LogInErr {
  UserNotExist,
  PasswdNotMatch,
}

pub struct ProspectSqlPool {
  pool: Pool<MySql>,
  rng: StdRng,
}

pub async fn new_pool(user: String, pass: String, addr: String, max: u32) -> Result<ProspectSqlPool, sqlx::Error> {
  let pool = MySqlPoolOptions::new()
    .max_connections(max)
    .connect(&format!("mysql://{}:{}@{}/mysql", user, pass, addr)).await?;

  // let row: (i64, ) = sqlx::query_as("SELECT ?")
  //   .bind(150_i64)
  //   .fetch_one(&pool).await?;

  // println!("{}", row.0);
  Ok(ProspectSqlPool {
    pool,
    rng: StdRng::from_entropy(),
  })
}

impl ProspectSqlPool {
  pub async fn sign_in(&mut self, info: SignUpInfo) -> Result<(), SignUpErr> {
    let (salt, pass_and_salt_hash) = self.KDF(&info.password);
    Ok(())
  }

  fn KDF(&mut self, passwd: &str) -> ([u8; 8], String) {
    let salt = self.rng.gen::<[u8; 8]>();
    let mut hasher = crypto::sha3::Sha3::sha3_256();
    let b = passwd.as_bytes()
      .iter()
      .chain(
        salt.as_slice()
          .iter()
      )
      .map(|x| *x)
      .collect::<Vec<u8>>();
    hasher.input(b.as_slice());
    (salt, hasher.result_str())
  }
}
