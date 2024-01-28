use actix_web::{web, App, HttpServer};
use aer::{args::Args, routes::route_config};
use clap::Parser;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let shared_args = Arc::new(args);

    println!("Starting server at http://localhost:9999");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_args.clone()))
            .configure(route_config)
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
