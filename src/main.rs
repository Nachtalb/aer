use actix_web::{middleware::Logger, web, App, HttpServer};
use aer::{args::Args, routes::route_config};
use clap::Parser;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let shared_args = Arc::new(args);

    println!("Starting server at http://localhost:9999");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(shared_args.clone()))
            .configure(route_config)
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
