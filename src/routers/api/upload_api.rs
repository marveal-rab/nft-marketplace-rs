use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::json;

use crate::middlewares::auth::CurrentUser;

pub async fn upload(
    Extension(current_user): Extension<CurrentUser>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
    return Json(json!({ "url": "test url" }));
}
