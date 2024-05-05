use std::io::Read;

use async_graphql::{Context, Upload};

use crate::errors::AppError;

pub fn parse_upload(ctx: &Context<'_>, file: Upload) -> Result<(String, Vec<u8>), AppError> {
    let file_name = file.value(ctx).unwrap().filename;
    let mut buffer = Vec::new();
    file.value(ctx)
        .unwrap()
        .content
        .read_to_end(&mut buffer)
        .map_err(|err| {
            tracing::error!("upload file error: {:?}", err);
            AppError::UploadMissingFile
        })?;
    Ok((file_name, buffer))
}
