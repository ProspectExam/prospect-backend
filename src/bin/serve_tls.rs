#![feature(addr_parse_ascii)]
#![feature(async_closure)]

use std::borrow::BorrowMut;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::net::SocketAddr;
use std::convert::Infallible;

use argh::FromArgs;
use warp::Filter;
use rustls_pemfile::read_one;
use sqlx::{MySql, Pool};


use prospect_backend::types::*;
use prospect_backend::users;
use prospect_backend::users::{ProspectSqlPool, SignUpErr};

/// warp TLS server example
#[derive(FromArgs)]
struct Options {
  /// bind addr
  #[argh(positional)]
  addr: String,

  /// cert file
  #[argh(option, short = 'c')]
  cert: String,

  /// key file
  #[argh(option, short = 'k')]
  key: String,

  /// sql user
  #[argh(option, short = 'u')]
  sql_user: String,

  /// sql ip
  #[argh(option, short = 'a')]
  sql_addr: String,

  /// sql passwd
  #[argh(option, short = 'p')]
  sql_passwd: String,

  /// echo mode
  #[argh(switch, short = 'e')]
  echo_mode: bool,
}

// TODO: do not use Mutex
type PPool = Arc<tokio::sync::Mutex<ProspectSqlPool>>;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();
  let options: Options = argh::from_env();

  let pool = Arc::new(tokio::sync::Mutex::new(
    users::new_pool(
      options.sql_user.clone(),
      options.sql_passwd.clone(),
      options.sql_addr,
      5
    ).await.unwrap())
  );

  let root = warp::any();
  let hello_world = root
    .and(warp::get())
    .and(warp::path("hello"))
    .map(|| {
      "<html>\n\
     <head> hello world </head>\n\
     </html>\n\
     <body>\n\
     <h1>Hello world!</h1>\n\
     </br>\n\
     A test server wrapped by cloudflare and running with TLS\n\
     </body>"
    });

  let pcp = Arc::clone(&pool);
  let post_route = root
    .and(warp::post())
    .and(warp::path("sign_up"))
    .and(warp::path::end())
    // .and(warp::path::param::<u32>())
    .and(warp::body::content_length_limit(1024 << 4))
    .and(warp::body::json())
    // .map(|mut info: SignUpInfo| {
    //   info.password = "114514".to_string();
    //   warp::reply::json(&info)
    // })
    .and(with_pool(Arc::clone(&pool)))
    .and_then(sign_up);


  let get_route = root
    .and(warp::get())
    .and(warp::path("sign_in"))
    .and(warp::path::end())
    .map(|| {
      warp::http::Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Foo", "Bar")
        .body(
          "<form method=\"post\" action=\"sign_in\">\n\
      Username: <input type=\"text\" name=\"username\">\n\
      Password: <input type=\"password\" name=\"password\">\n\
      <input type=\"submit\" value=\"Submit\">
      </form>
      ")
    });

  let routes = warp::any().and(post_route.or(hello_world).or(get_route));

  warp::serve(routes)
    .tls()
    .cert_path(&options.cert)
    .key_path(&options.key)
    // .run(([0, 0, 0, 0], 443))
    .run(SocketAddr::parse_ascii((&options.addr).as_ref()).unwrap())
    .await;
}

async fn sign_up(info: SignUpInfo, mut pool: PPool) -> Result<impl warp::Reply, Infallible> {
  let mut reply = SignUpResult { success: false, message: "".to_string() };
  match pool.lock().await.borrow_mut().sign_in(info).await {
    Ok(_) => {
      reply.success = true;
      reply.message = "".to_string();
    }
    Err(e) => {
      reply.success = false;
      reply.message = match e {
        SignUpErr::UserExist => "user exist".to_string(),
      }
    }
  }
  Ok(warp::reply::json(&reply))
}

fn with_pool(pool: PPool) -> impl Filter<Extract = (PPool,), Error = Infallible> + Clone {
  warp::any().map(move || pool.clone())
}
