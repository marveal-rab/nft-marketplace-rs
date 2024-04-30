use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use ipfs_api_backend_hyper::IpfsClient;
use serde_json::json;

use crate::middlewares::auth::CurrentUser;

pub async fn upload(
    Extension(current_user): Extension<CurrentUser>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let client = IpfsClient::default();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", file_name, data.len());
    }
    return Json(json!({ "url": "test url" }));
}
