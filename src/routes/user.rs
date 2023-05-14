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

    diesel::insert_into(users::table)
        .values::<&User>(&user)
        .returning(users::user_name)
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({
            "success": format!("User {} created!", user.user_name)
        })),
        StatusCode::CREATED,
    ))
}
