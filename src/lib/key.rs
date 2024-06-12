use std::time::{SystemTime, UNIX_EPOCH};

use super::config::HttpServerConfig;

#[derive(Debug)]
pub struct DecryptionError;

impl std::fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decryption error")
    }
}

impl std::error::Error for DecryptionError {}

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
//私钥
const KEY: &[u8; 16] = b"1234567890abcdef";
//初始化向量
const IV: &[u8; 16] = b"1234567890abcdef";

pub struct Key {
    key: [u8; 16],
    iv: [u8; 16],
}

use base64::{engine::general_purpose, Engine};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTimeTokenStruct {
    user: u64,
    time: u128,
}

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use md5::{Digest, Md5};

// 生成md5
pub fn generate_md5_key(text: &str) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(text);
    let result = hasher.finalize();
    result.into()
}
// md5转换为字符串
pub fn generate_md5_key_tostr(text: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(text);
    let result = hasher.finalize();
    format!("{:x}", result)
}

impl Default for Key {
    fn default() -> Self {
        let config = HttpServerConfig::default();
        let key = generate_md5_key(&config.name);
        let iv = generate_md5_key(&config.sql_url);
        Key { key, iv }
    }
}

/// 普通加密
pub fn encrypt(plain: &[u8]) -> Result<Vec<u8>, DecryptionError> {
    let iv: [u8; 16] = Key::default().iv;
    let key: &[u8; 16] = &Key::default().key;
    // 创建明文缓冲区并复制明文到其中
    let mut plain_buf: Vec<u8> = Vec::with_capacity(plain.len());
    plain_buf.extend_from_slice(plain);

    // 创建密文缓冲区
    let mut ct_buf: Vec<u8> = vec![0u8; plain.len() + 16]; // 初始容量为明文长度 + 16（iv长度）

    // 进行加密
    let ct = match Aes128CbcEnc::new(key.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&plain_buf, &mut ct_buf)
    {
        Ok(pt) => pt,
        Err(_) => return Err(DecryptionError),
    };
    Ok(ct.to_vec())
}

/// 普通解密
pub fn decrypt(cipher: &[u8]) -> Result<Vec<u8>, DecryptionError> {
    let iv: [u8; 16] = Key::default().iv;
    let key: &[u8; 16] = &Key::default().key;

    // 创建明文缓冲区并复制明文到其中
    let mut plain_buf: Vec<u8> = Vec::with_capacity(cipher.len());
    plain_buf.extend_from_slice(cipher);

    // 创建密文缓冲区
    let mut ct_buf: Vec<u8> = vec![0u8; cipher.len() + 16];

    // 进行解密并处理错误
    let pt = match Aes128CbcDec::new(key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(cipher, &mut ct_buf)
    {
        Ok(pt) => pt,
        Err(_) => return Err(DecryptionError),
    };
    Ok(pt.to_vec())
}

//生成临时token值
pub fn generate_token(user: u64) -> Result<String, DecryptionError> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let user = UserTimeTokenStruct { user, time };
    match serde_json::to_string(&user) {
        Ok(_datestr) => {
            let encrypt_str = encrypt(_datestr.as_bytes()).unwrap();
            Ok(general_purpose::STANDARD.encode(&encrypt_str))
        }
        Err(_) => Err(DecryptionError),
    }
}

//token值验证
pub fn gettoken_to_user(user_token_text: String) -> Result<u64, DecryptionError> {
    let bytes = general_purpose::STANDARD.decode(&user_token_text);
    match bytes {
        Ok(bytes) => {
            let decrypt_str = decrypt(&bytes).unwrap();
            let user_str = String::from_utf8(decrypt_str.to_vec()).unwrap();
            match serde_json::from_str::<UserTimeTokenStruct>(&user_str) {
                Ok(user_time_token_struct) => {
                    let time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let old_time = user_time_token_struct.time;
                    let _now_time_s = (time - old_time) / 1000; //秒
                    let _now_time_min = (time - old_time) / (1000 * 60); //分
                    let now_time_h = (time - old_time) / (1000 * 60 * 60); //小时

                    //token超过十二个小时无效
                    if now_time_h <= 12 {
                        Ok(user_time_token_struct.user)
                    } else {
                        Err(DecryptionError)
                    }
                }
                Err(_err) => Err(DecryptionError),
            }
        }
        Err(_err) => Err(DecryptionError),
    }
}

use jsonwebtoken::decode as json_decode;
use jsonwebtoken::encode as json_encode;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenUser {
    pub uid: u64,
    pub(crate) exp: u64,
}

pub const TOKEN_KEY_STR: &str = "1234567890abcdef";
// 解密token值
pub fn gettoken_to_user_no_time(
    token: &str,
) -> Result<TokenData<TokenUser>, jsonwebtoken::errors::Error> {
    json_decode::<TokenUser>(
        &token,
        &DecodingKey::from_secret(TOKEN_KEY_STR.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}


// 为user生成一个token，有效期time秒
pub fn create_token_time_s(user: u64, time: u128) -> String {
    json_encode(
        &Header::default(),
        &TokenUser {
            uid: user,
            exp: (Utc::now() + Duration::seconds(time as i64)).timestamp() as u64,
        },
        &EncodingKey::from_secret(TOKEN_KEY_STR.as_ref()),
    )
    .unwrap()
}

// 为user生成一个token，有效期time分
pub fn create_token_time_min(user: u64, time: u128) -> String {
    json_encode(
        &Header::default(),
        &TokenUser {
            uid: user,
            exp: (Utc::now() + Duration::minutes(time as i64)).timestamp() as u64,
        },
        &EncodingKey::from_secret(TOKEN_KEY_STR.as_ref()),
    )
    .unwrap()
}

// 为user生成一个token，有效期time小时
pub fn create_token_time_h(user: u64, time: u128) -> String {
    json_encode(
        &Header::default(),
        &TokenUser {
            uid: user,
            exp: (Utc::now() + Duration::hours(time as i64)).timestamp() as u64,
        },
        &EncodingKey::from_secret(TOKEN_KEY_STR.as_ref()),
    )
    .unwrap()
}

// 为user生成一个token，有效期time天
pub fn create_token_time_day(user: u64, time: u128) -> String {
    json_encode(
        &Header::default(),
        &TokenUser {
            uid: user,
            exp: (Utc::now() + Duration::days(time as i64)).timestamp() as u64,
        },
        &EncodingKey::from_secret(TOKEN_KEY_STR.as_ref()),
    )
    .unwrap()
}


#[tokio::test]
async fn test_key() {
    let user = 1;
    let token = create_token_time_h(user, 12);
    println!("token: {}", token);
    let user = gettoken_to_user_no_time(&token).unwrap();
    assert_eq!(user.claims.uid, 1);
}