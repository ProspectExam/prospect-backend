#![feature(addr_parse_ascii)]
#![feature(async_closure)]

use std::net::SocketAddr;
use std::sync::Arc;
use std::convert::Infallible;

use log::{info, LevelFilter};
use warp::Filter;

use prospect_backend::database::ProspectSqlPool;
use prospect_backend::wechat::{types::*, handlers::*};

#[tokio::main]
async fn main() {
  pretty_env_logger::formatted_timed_builder()
    .format_timestamp_secs()
    .filter_level(LevelFilter::Info)
    .init();

  let options: Options = argh::from_env();
  info!("Prospect server_wx start");
  ProspectSqlPool::init(
    options.sql_user.clone(),
    options.sql_passwd.clone(),
    options.sql_addr.clone(),
  ).await.unwrap();
  let pool = ProspectSqlPool::new(
    options.sql_user.clone(),
    options.sql_passwd.clone(),
    options.sql_addr.clone(),
    "Prospect".to_string(),
    5,
  ).await.unwrap();
  if options.init_from_fs {
    pool.init_from_assets(options.assets_path.clone().into()).await.unwrap();
  }
  info!("Create Sql connection pool OK");

  let ctx = Context::new(pool, Arc::new(options.clone()));
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

  // send_code route
  let route_send_code = root
    .and(warp::post())
    .and(warp::path("send_code"))
    .and(warp::path::end())
    .and(warp::body::content_length_limit(4096))
    .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(send_code_handler);
  info!("Path \"/send_code\" created");

  // waterfall route
  let route_waterfall = root
    .and(warp::get())
    .and(warp::path("waterfall"))
    .and(warp::path::end())
    // .and(warp::body::content_length_limit(4096))
    // .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(waterfall_handler);
  info!("Path \"/waterfall\" created");

  // subscribe route
  let route_subscribe = root
    .and(warp::post())
    .and(warp::path("subscribe"))
    .and(warp::path::end())
    .and(warp::body::content_length_limit(4096))
    .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(subscribe_handler);
  info!("Path \"/subscribe\" created");

  // /get_user_subscribe route
  let route_get_user_subscribe = root
    .and(warp::post())
    .and(warp::path("get_user_subscribe"))
    .and(warp::path::end())
    .and(warp::body::content_length_limit(4096))
    .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(get_user_subscribe_handler);
  info!("Path \"/get_user_subscribe\" created");

  // get_university_info route
  let route_get_university = root
    .and(warp::get())
    .and(warp::path("get_university"))
    .and(warp::path::end())
    .and(with_context(ctx.clone()))
    .and_then(get_university_handler);
  info!("Path \"/get_university\" created");

  // get_department route
  let route_get_department = root
    .and(warp::post())
    .and(warp::path("get_department"))
    .and(warp::path::end())
    .and(warp::body::content_length_limit(4096))
    .and(warp::body::json())
    .and(with_context(ctx.clone()))
    .and_then(get_department_handler);
  info!("Path \"/get_department\" created");

  // post of assets
  let route_assets_article = root
    .and(warp::get())
    .and(warp::path("post"))
    .and(warp::fs::dir(options.assets_path.clone() + "/post"));

  // paper of assets
  let route_assets_paper = root
    .and(warp::get())
    .and(warp::path("paper"))
    .and(warp::fs::dir(options.assets_path.clone() + "/paper"));

  let routes = warp::any()
    .and(hello_world)
    .or(route_send_code)
    .or(route_waterfall)
    .or(route_subscribe)
    .or(route_get_user_subscribe)
    .or(route_get_university)
    .or(route_get_department)
    .or(route_assets_article)
    .or(route_assets_paper);
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
