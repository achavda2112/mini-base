use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::model::{TokenFile, TokenUser, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenClaims {
    pub user: TokenUser,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageTokenClaims {
    pub file: TokenFile,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_auth_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_in = Duration::hours(24);
    now += exp_in;
    let exp = now.timestamp() as usize;

    let token_user = TokenUser {
        id: user.id,
        email: user.email,
        role: user.role,
    };

    let claim = AuthTokenClaims {
        exp,
        iat,
        user: token_user,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

pub fn decode_auth_token(
    token: &str,
) -> Result<TokenData<AuthTokenClaims>, jsonwebtoken::errors::Error> {
    decode::<AuthTokenClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
}

pub fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

pub fn generate_storage_token(
    token_file: TokenFile,
) -> Result<String, jsonwebtoken::errors::Error> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_in = Duration::minutes(2);
    now += exp_in;
    let exp = now.timestamp() as usize;

    let claim = StorageTokenClaims {
        exp,
        iat,
        file: token_file,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

pub fn decode_storage_token(
    token: &str,
) -> Result<TokenData<StorageTokenClaims>, jsonwebtoken::errors::Error> {
    decode::<StorageTokenClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
}
