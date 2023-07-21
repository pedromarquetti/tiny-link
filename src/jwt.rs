use crate::db::User;
use jsonwebtoken::{
    decode, encode,
    errors::{Error as JWTError, Result as JWTResult},
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use std::env;

fn get_secret() -> String {
    return env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found!");
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_name: String,
    pub role: String,
    pub exp: usize,
}
/* creates new token for provided user */
pub fn generate_token(usr: User) -> Result<String, JWTError> {
    let claims = UserClaims {
        user_name: usr.user_name.to_owned(),
        role: usr.user_role.to_owned().unwrap(),
        // exp: time.into(),
        exp: 10000000000,
    };

    return encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    );
}

pub fn validate_token<T: DeserializeOwned>(token: String) -> JWTResult<TokenData<T>> {
    return decode(
        &token,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::new(Algorithm::HS256),
    );
}
