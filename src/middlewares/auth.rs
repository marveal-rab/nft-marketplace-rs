use aes::cipher::BlockDecrypt;
use aes::cipher::{generic_array::GenericArray, KeyInit};
use aes::Aes256;
use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use base64::{engine::general_purpose, Engine};

#[derive(Clone)]
pub struct CurrentUser {
    address: String,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    // KAK9vx7DRU1v6TdHSV4pzDRnTLhfadeMjz6iLY448dZRFrkjp1aDwbeJ5Su7hDsB
    // 解码 base64 编码的密文
    let encrypted_bytes = general_purpose::STANDARD
        .decode(auth_token)
        .map_err(|e| format!("Base64 decoding error: {}", e))
        .unwrap();

    // 创建 AES 解密器
    let key = GenericArray::from_slice(b"bW5pRKLlkdMOe028l0vOqGmKM87KhUfC");
    let cipher = Aes256::new(&key);

    // 解密
    let mut decrypted = vec![0u8; encrypted_bytes.len()];
    let blocks = encrypted_bytes.chunks_exact(16);
    for (i, block) in blocks.enumerate() {
        let mut block = GenericArray::clone_from_slice(block);
        cipher.decrypt_block(&mut block);
        decrypted[i * 16..(i + 1) * 16].copy_from_slice(&block);
    }

    // 移除填充
    let padding_length = decrypted[decrypted.len() - 1] as usize;
    decrypted.truncate(decrypted.len() - padding_length);

    // 转换为字符串
    let decrypted_str = String::from_utf8(decrypted)
        .map_err(|e| format!("Decryption result is not valid UTF-8: {}", e))
        .unwrap();

    return Some(CurrentUser {
        address: decrypted_str,
    });
}
