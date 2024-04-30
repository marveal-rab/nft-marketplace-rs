use axum::body::Bytes;
use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::Extension;
use reqwest::Client;
use serde::Serialize;

use crate::errors::AppError;
use crate::middlewares::auth::CurrentUser;
use crate::routers::ApiResponse;

async fn ipfs_file_upload(file_name: String, file_bytes: Vec<u8>) -> Result<String, AppError> {
    let client = Client::new();
    let ipfs_api_endpoint = "http://127.0.0.1:5001/api/v0/add";

    return match client
        .post(ipfs_api_endpoint)
        .multipart(reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes).file_name(file_name),
        ))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(response_body) => {
                        match serde_json::from_str::<serde_json::Value>(&response_body) {
                            Ok(response_body_json) => {
                                let ipfs_url = format!(
                                    "https://ipfs.io/ipfs/{}",
                                    response_body_json["Hash"].as_str().unwrap_or_default()
                                );
                                Ok(ipfs_url.to_string())
                            }
                            Err(err) => {
                                tracing::error!(
                                    "request ipfs add response body deserialize failed: {:?}",
                                    err
                                );
                                Err(AppError::RequestIpfsAddResponseBodyDeserializeFailed)
                            }
                        }
                    }
                    Err(err) => {
                        tracing::error!("request ipfs add response no body: {:?}", err);
                        Err(AppError::RequestIpfsAddResponseNoBody)
                    }
                }
            } else {
                Err(AppError::RequestIpfsAddFailed)
            }
        }
        Err(err) => {
            tracing::error!("request ipfs add error: {:?}", err);
            Err(AppError::RequestIpfsAddError)
        }
    };
}

#[derive(Serialize)]
pub struct UploadResponse {
    url: String,
}

pub async fn upload(
    Extension(current_user): Extension<CurrentUser>,
    _headers: HeaderMap,
    mut multipart: Multipart,
) -> ApiResponse<UploadResponse> {
    // TODO check current user address
    let address = current_user.address;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data: Bytes = field.bytes().await.unwrap();

        return match ipfs_file_upload(file_name, data.to_vec()).await {
            Ok(url) => {
                // TODO record to db

                let upload_response = UploadResponse { url };
                ApiResponse::success_with_data(upload_response)
            }
            Err(err) => ApiResponse::error(err),
        };
    }
    ApiResponse::error(AppError::UploadMissingFile)
}
