use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// メールアドレスをHMAC-SHA256でハッシュ化（決定論的 — ログイン時の照合に使用）
pub fn hash_email(email: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC accepts any key size");
    mac.update(email.to_lowercase().trim().as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// パスワードをArgon2idでハッシュ化（ランダムソルト）
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// パスワードの照合
pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// セッショントークンをSHA-256でハッシュ化（DB保存用）
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// 安全なランダムセッショントークンを生成（64文字hex）
pub fn generate_token() -> String {
    let token: [u8; 32] = rand::thread_rng().gen();
    hex::encode(token)
}
