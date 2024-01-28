use crate::{args, rreaction, utils};
use actix_web::{web, HttpResponse, Responder};
use redis::Commands;
use serde::Serialize;
use std::ops::DerefMut;
use std::{path::Path, sync::Arc};

#[derive(Serialize)]
struct FileInfo {
    url: String,
    path: String,
    name: String,
    md5: String,
    type_: String,
    tags: Vec<String>,
    full_path: String,
}

impl FileInfo {
    fn builder() -> FileInfoBuilder {
        FileInfoBuilder::default()
    }
}

#[derive(Default)]
struct FileInfoBuilder {
    url: Option<String>,
    path: Option<String>,
    name: Option<String>,
    md5: Option<String>,
    type_: Option<String>,
    tags: Option<Vec<String>>,
    full_path: Option<String>,
}

impl FileInfoBuilder {
    fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn md5(mut self, md5: String) -> Self {
        self.md5 = Some(md5);
        self
    }

    fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }

    fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    fn full_path(mut self, full_path: String) -> Self {
        self.full_path = Some(full_path);
        self
    }

    fn build(self) -> FileInfo {
        FileInfo {
            url: self.url.unwrap(),
            path: self.path.unwrap(),
            name: self.name.unwrap(),
            md5: self.md5.unwrap(),
            type_: self.type_.unwrap(),
            tags: self.tags.unwrap(),
            full_path: self.full_path.unwrap(),
        }
    }
}

fn file_info(
    path: &Path,
    base_path: &Path,
    conn: &mut r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>,
) -> Result<FileInfo, ()> {
    let mut conn = conn.deref_mut();

    let path_no_base = utils::relative_to(path, base_path);

    let key = format!("type:{}", path_no_base.to_str().unwrap());
    let md5 = match conn.get(key) {
        Ok(md5) => md5,
        Err(_) => {
            let md5 = utils::compute_md5(path).unwrap();
            conn.set_ex(key, md5.clone()).unwrap();
            md5
        }
    };

    Ok(FileInfo::builder()
        .name(
            path_no_base
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        )
        .path(path_no_base.to_str().unwrap().to_string())
        .full_path(path.to_str().unwrap().to_string())
        .url(format!(
            "http://localhost:9990/{}",
            path_no_base.to_str().unwrap()
        ))
        .md5(utils::compute_md5(path).unwrap())
        .build())
}

#[actix_web::get("/files")]
pub async fn files_index(
    data: web::Data<Arc<args::Args>>,
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
) -> impl Responder {
    let mut conn = match redis_pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            log::warn!("Failed to get Redis connection from pool");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let files = rreaction::get_files(data.path.clone())
        .iter()
        .flat_map(|path| file_info(path, &data.path, &mut conn))
        .collect::<Vec<FileInfo>>();
    HttpResponse::Ok().json(files)
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
    HttpResponse::Ok().json(files)
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
            redis_host: "localhost".to_string(),
            redis_port: 6379,
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
            redis_host: "localhost".to_string(),
            redis_port: 6379,
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
            redis_host: "localhost".to_string(),
            redis_port: 6379,
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
