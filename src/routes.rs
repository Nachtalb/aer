use crate::{files, index};
use actix_web::web;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(files::files_upload)
        .service(files::files_index)
        .service(files::files_filter);
}
