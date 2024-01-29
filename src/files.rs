use crate::{args, rreaction, utils};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum FileType {
    #[serde(rename = "image")]
    Image,

    #[serde(rename = "video")]
    Video,

    #[serde(rename = "audio")]
    Audio,

    #[serde(rename = "other")]
    Other,
}

impl FileType {
    fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        match extension {
            "jpg" | "jpeg" | "png" | "gif" | "webp" => Some(Self::Image),
            "mp4" | "webm" => Some(Self::Video),
            "mp3" | "flac" => Some(Self::Audio),
            _ => Some(Self::Other),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FileInfo {
    url: String,
    name: String,
    path: String,
    type_: FileType,
    extension: String,
    hash: String,
}

impl FileInfo {
    fn from_path(path: &Path, base: &Path) -> Self {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let relative_path = utils::relative_to(path, base).to_str().unwrap().to_string();
        let url = format!("/media/{}", relative_path);

        let type_ = FileType::from_path(relative_path.as_ref()).unwrap();
        let extension = path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_lowercase();

        let hash = utils::fast_hash_str(&relative_path);

        Self {
            url,
            name,
            path: relative_path,
            type_,
            extension,
            hash,
        }
    }
}

#[actix_web::get("/files")]
pub async fn files_index(data: web::Data<Arc<args::Args>>) -> impl Responder {
    let files = &rreaction::get_files(data.path.clone())
        .iter()
        .map(|p| FileInfo::from_path(p, &data.path))
        .collect::<Vec<_>>();
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
    let files = &rreaction::get_files_with_filter(data.path.clone(), types)
        .iter()
        .map(|p| FileInfo::from_path(p, &data.path))
        .collect::<Vec<_>>();
    HttpResponse::Ok().json(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use std::path::PathBuf;

    #[test]
    async fn test_file_type_from_path() {
        let path = Path::new("test_data/1.jpg");
        assert_eq!(FileType::from_path(path), Some(FileType::Image));

        let path = Path::new("test_data/1.png");
        assert_eq!(FileType::from_path(path), Some(FileType::Image));

        let path = Path::new("test_data/1.gif");
        assert_eq!(FileType::from_path(path), Some(FileType::Image));

        let path = Path::new("test_data/1.webp");
        assert_eq!(FileType::from_path(path), Some(FileType::Image));

        let path = Path::new("test_data/1.webm");
        assert_eq!(FileType::from_path(path), Some(FileType::Video));

        let path = Path::new("test_data/1.mp4");
        assert_eq!(FileType::from_path(path), Some(FileType::Video));

        let path = Path::new("test_data/1.flac");
        assert_eq!(FileType::from_path(path), Some(FileType::Audio));

        let path = Path::new("test_data/1.mp3");
        assert_eq!(FileType::from_path(path), Some(FileType::Audio));

        let path = Path::new("test_data/1.txt");
        assert_eq!(FileType::from_path(path), Some(FileType::Other));

        let path = Path::new("test_data/1");
        assert_eq!(FileType::from_path(path), None);
    }

    #[test]
    async fn test_file_info_from_path() {
        let path = Path::new("test_data/1.jpg");
        let base = Path::new("test_data");
        let info = FileInfo::from_path(path, base);
        assert_eq!(info.url, "/media/1.jpg");
        assert_eq!(info.name, "1.jpg");
        assert_eq!(info.path, "1.jpg");
        assert_eq!(info.type_, FileType::Image);
        assert_eq!(info.extension, "jpg");

        let path = Path::new("test_data/1.mp4");
        let base = Path::new("test_data");
        let info = FileInfo::from_path(path, base);
        assert_eq!(info.url, "/media/1.mp4");
        assert_eq!(info.name, "1.mp4");
        assert_eq!(info.path, "1.mp4");
        assert_eq!(info.type_, FileType::Video);
        assert_eq!(info.extension, "mp4");
    }

    #[actix_web::test]
    async fn test_files_index() {
        let args = crate::args::Args {
            path: PathBuf::from("test_data"),
        };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(args)))
                .configure(crate::routes::route_config),
        )
        .await;
        let req = test::TestRequest::with_uri("/files").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let files: Vec<FileInfo> = serde_json::from_slice(&body).unwrap();
        assert_eq!(files.len(), 7);
    }

    #[actix_web::test]
    async fn test_files_filter() {
        let args = crate::args::Args {
            path: PathBuf::from("test_data"),
        };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(args)))
                .configure(crate::routes::route_config),
        )
        .await;
        let req = test::TestRequest::with_uri("/files/gif").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let files: Vec<FileInfo> = serde_json::from_slice(&body).unwrap();
        assert_eq!(files.len(), 1);
    }

    #[actix_web::test]
    async fn test_files_filter_invalid() {
        let args = crate::args::Args {
            path: PathBuf::from("test_data"),
        };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(args)))
                .configure(crate::routes::route_config),
        )
        .await;
        let req = test::TestRequest::with_uri("/files/invalid").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Invalid filter");
    }
}
