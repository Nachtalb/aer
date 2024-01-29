use reqwest::{multipart, Body};
use std::path::Path;
use tokio::{self, fs::File};
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn upload_file(path: &Path, url: String, token: String) -> Result<String, ()> {
    let client = reqwest::Client::new();
    let file = File::open(path).await.map_err(|_| ())?;

    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    let upload_file = multipart::Part::stream(file_body)
        .file_name(path.file_name().unwrap().to_str().unwrap().to_string());

    let form = multipart::Form::new().part("file", upload_file);

    let res = client
        .post(&url)
        .header("Authorization", token)
        .multipart(form)
        .send()
        .await
        .map_err(|_| ())?
        .error_for_status()
        .map_err(|_| ())?;

    res.text().await.map_err(|_| ())
}
