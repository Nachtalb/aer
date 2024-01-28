use actix_files as fs;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use rreaction_search::{args::Args, routes::route_config};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let shared_args = Arc::new(args);

    println!("Starting server at http://localhost:9999");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_args.clone()))
            .service(fs::Files::new("/media/", shared_args.path.clone()).show_files_listing())
            .configure(route_config)
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
