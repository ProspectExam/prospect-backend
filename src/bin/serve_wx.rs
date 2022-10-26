#![feature(addr_parse_ascii)]
#![feature(async_closure)]

use std::borrow::BorrowMut;
use std::net::SocketAddr;
use std::sync::Arc;
use std::convert::Infallible;

use argh::FromArgs;
use log::{info, LevelFilter};
use warp::Filter;

use prospect_backend::types::LogInInfo;
use prospect_backend::users::{self, ProspectSqlPool};
use prospect_backend::wechat::{types::*, handlers::*};

#[tokio::main]
async fn main() {
  pretty_env_logger::formatted_timed_builder()
    .format_timestamp_secs()
    .filter_level(LevelFilter::Debug)
    .init();

  let options: Options = argh::from_env();
  info!("Prospect server_wx start");
  let pool = Arc::new(tokio::sync::Mutex::new(
    users::new_pool(
      options.sql_user.clone(),
      options.sql_passwd.clone(),
      options.sql_addr.clone(),
      5,
    ).await.unwrap()
  ));
  info!("Create Sql connection pool OK");

  let ctx = Context {
    pool,
    options: Arc::new(options),
  };
  println!("{:?}", ctx);

  let root = warp::any();
  let hello_world = root
    .and(warp::get())
    .and(warp::path::end())
    .map(|| {
      warp::reply::html(
        "<html>\n\
         <head> <title> hello world </title> </head>\n\
         <body> <h1> hello world! </h1> </body>\n\
         </html>
        "
      )
    });
  info!("Path \"/\" created");

  let route_send_code = root
    .and(warp::post())
    .and(warp::path("send_code"))
    .and(warp::path::end())
    .and(warp::body::content_length_limit(4096))
    .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(send_code_handler);
  info!("Path \"/send_code\" created");

  let route_waterfall = root
    .and(warp::get())
    .and(warp::path("waterfall"))
    .and(warp::path::end())
    // .and(warp::body::content_length_limit(4096))
    // .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(waterfall_handler);
  info!("Path \"/waterfall\" created");

  let routes = warp::any()
    .and(hello_world)
    .or(route_send_code)
    .or(route_waterfall);
  info!("all route registered");
  info!("starting serve");

  warp::serve(routes)
    .tls()
    .cert_path(&ctx.options.cert)
    .key_path(&ctx.options.key)
    .run(SocketAddr::parse_ascii((&ctx.options.addr).as_ref()).unwrap())
    .await;
}

fn with_context(ctx: Context) -> impl Filter<Extract=(Context, ), Error=Infallible> + Clone {
  warp::any().map(move || ctx.clone())
}
