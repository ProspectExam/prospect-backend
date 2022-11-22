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
  info!("Create Sql connection pool OK");

  // random init some data
  pool.add_university("tsinghua", "清华大学").await; // 0
  pool.add_university("peking", "北京大学").await;   // 1
  pool.add_university("muc", "中央民族大学").await;  // 2
  pool.add_university("tongji", "同济大学").await;  // 3
  pool.add_university("sichuan", "四川大学").await; // 4
  pool.add_university("hunan", "湖南大学").await;   // 5
  pool.add_university("tianjin", "天津大学").await; // 6
  pool.add_university("beili", "北京理工大学").await; // 7
  pool.add_university("harbin", "哈尔滨工业大学").await; // 8

  pool.add_department(3, "cs", "计算机科学与技术").await; // 0
  pool.add_department(3, "ee", "电子工程").await; // 1
  pool.add_department(3, "me", "机械工程").await; // 2
  pool.add_department(3, "ce", "土木工程").await; // 3
  pool.add_department(3, "se", "软件工程").await; // 4
  pool.add_department(3, "ie", "信息工程").await; // 5
  pool.add_department(3, "ae", "自动化工程").await; // 6
  pool.add_department(3, "pe", "物理工程").await; // 7
  pool.add_department(3, "ch", "化学工程").await; // 8
  pool.add_department(3, "me", "材料工程").await; // 9
  pool.add_department(3, "me", "数学与应用数学").await; // 10
  pool.add_department(3, "me", "统计学").await; // 11
  pool.add_department(3, "me", "经济学").await; // 12
  pool.add_department(1, "se", "软件工程").await; // 4
  pool.add_department(1, "ee", "电子工程").await; // 5
  pool.add_department(1, "me", "机械工程").await; // 6
  pool.add_department(1, "ce", "土木工程").await; // 7
  pool.add_department(2, "cs", "计算机科学与技术").await; // 8
  pool.add_department(2, "ee", "电子工程").await; // 9
  pool.add_department(2, "me", "机械工程").await; // 10
  pool.add_department(2, "ce", "土木工程").await; // 11

  let ctx = Context::new(pool, Arc::new(options));
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

  let routes = warp::any()
    .and(hello_world)
    .or(route_send_code)
    .or(route_waterfall)
    .or(route_subscribe)
    .or(route_get_user_subscribe)
    .or(route_get_university)
    .or(route_get_department);
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
