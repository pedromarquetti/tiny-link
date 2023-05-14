use std::format;

use diesel::RunQueryDsl;
use hyper::StatusCode;
use serde_json::json;
use warp::{Rejection, Reply};

use crate::{
    db::{DbConnection, User},
    error::convert_to_rejection,
};

pub async fn create_user(user: User, conn: DbConnection) -> Result<impl Reply, Rejection> {
    use crate::schema::users;

    let mut conn = conn.map_err(convert_to_rejection)?;

    let payload: User = User {
        user_name: user.user_name,
        user_role: user.user_role,
        user_pwd: user.user_pwd,
    };
    diesel::insert_into(users::table)
        .values::<&User>(&payload)
        .returning(users::user_name)
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({
            "success": format!("User {} created!", payload.user_name)
        })),
        StatusCode::CREATED,
    ))
}
