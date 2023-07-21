use hyper::StatusCode;
use jsonwebtoken::TokenData;
use warp::{Filter, Rejection};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::{validate_token, UserClaims},
};

pub fn auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    // warp::cookie::warp::cookie::<String>("token")
    warp::cookie::optional::<String>("jwt")
        .and_then(check_header)
        .untuple_one()
}
// async fn check_header(header: Option<String>) -> Result<(), Rejection> {
async fn check_header(cookie: Option<String>) -> Result<(), Rejection> {
    // async fn check_header(cookie: String) -> Result<(), Rejection> {

    if let Some(cookie_val) = cookie {
        let token: TokenData<UserClaims> =
            validate_token(cookie_val).map_err(convert_to_rejection)?;
        if token.claims.role == "adm".to_string() {
            return Ok(());
        } else {
            return Err(
                Error::custom("Current user can't use this!", StatusCode::UNAUTHORIZED).into(),
            );
        }
    } else {
        return Err(Error::custom(
            "Unauthorized user! No Cookie Found ",
            StatusCode::UNAUTHORIZED,
        )
        .into());
    }
}
