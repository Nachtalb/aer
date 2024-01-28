use actix_web::Responder;

#[actix_web::get("/")]
pub async fn index() -> impl Responder {
    actix_files::NamedFile::open("static/index.html")
}
