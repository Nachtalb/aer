use actix_web::{middleware::Logger, web, App, HttpServer};
use aer::{args::Args, routes::route_config};
use clap::Parser;
use r2d2_redis::RedisConnectionManager;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let shared_args = Arc::new(args);

    let manager = RedisConnectionManager::new(format!(
        "redis://{}:{}/",
        shared_args.redis_host, shared_args.redis_port
    ))
    .expect("Failed to create Redis connection manager.");

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server at http://localhost:9999");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(shared_args.clone()))
            .configure(route_config)
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
