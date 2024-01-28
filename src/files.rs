use crate::{args, rreaction, utils};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

#[actix_web::get("/files")]
pub async fn files_index(data: web::Data<Arc<args::Args>>) -> impl Responder {
    let files = utils::all_relative_to(&rreaction::get_files(data.path.clone()), &data.path);
    let mut response = String::new();
    for file in files {
        response.push_str(&format!("{}\n", file.display()));
    }
    response
}

#[actix_web::get("/files/{filter}")]
pub async fn files_filter(
    data: web::Data<Arc<args::Args>>,
    filter: web::Path<String>,
) -> impl Responder {
    let types = match rreaction::default_type_builder().select(&filter).build() {
        Ok(types) => types,
        Err(_) => return HttpResponse::BadRequest().body("Invalid filter"),
    };
    let files = utils::all_relative_to(
        &rreaction::get_files_with_filter(data.path.clone(), types),
        &data.path,
    );
    let mut response = String::new();
    for file in files {
        response.push_str(&format!("{}\n", file.display()));
    }
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use std::path::PathBuf;

    #[actix_web::test]
    async fn test_files_index() {
        let args = crate::args::Args {
            path: PathBuf::from("test_data"),
        };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(args)))
                .configure(crate::routes::route_config),
        )
        .await;
        let req = test::TestRequest::with_uri("/files").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).expect("Failed to parse body");
        assert!(body_str.contains("file.txt"));
        assert!(body_str.contains("(!)(shock)(😮).jpg"));
    }

    #[actix_web::test]
    async fn test_files_filter() {
        let args = crate::args::Args {
            path: PathBuf::from("test_data"),
        };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(args)))
                .configure(crate::routes::route_config),
        )
        .await;
        let req = test::TestRequest::with_uri("/files/gif").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).expect("Failed to parse body");
        assert!(body_str.contains("(!)(shock)(😮).gif"));
    }

    #[actix_web::test]
    async fn test_files_filter_invalid() {
        let args = crate::args::Args {
            path: PathBuf::from("test_data"),
        };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(args)))
                .configure(crate::routes::route_config),
        )
        .await;
        let req = test::TestRequest::with_uri("/files/invalid").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
