use actix_web::Responder;

#[actix_web::get("/")]
pub async fn index() -> impl Responder {
    "Hello world!"
}

#[cfg(test)]
mod tests {
    use actix_web::{http, test, web, App};
    use std::path::PathBuf;
    use std::sync::Arc;

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
}
