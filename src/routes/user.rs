use std::format;

use bcrypt::{hash, verify};
use diesel::{
    result::{DatabaseErrorKind, Error as DieselError},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde_json::json;
use warp::{
    http::{header::SET_COOKIE, StatusCode},
    Rejection, Reply,
};

use crate::{
    db::{DbConnection, LoginUser, User},
    error::{convert_to_rejection, Error},
    jwt::generate_token,
};

pub async fn user_login(
    login_user: LoginUser,
    conn: DbConnection,
) -> Result<impl Reply, Rejection> {
    use crate::schema::users::dsl::*;

    let mut conn = conn.map_err(convert_to_rejection)?;
    let query: Result<User, DieselError> = users
        // checks if user exists
        .filter(user_name.eq(&login_user.user_name))
        // returns user pwd
        .get_result::<User>(&mut conn);

    match query {
        // user found
        Ok(result) => {
            // matching pwd
            if verify(&login_user.user_pwd, &result.user_pwd).map_err(convert_to_rejection)? {
                let token = generate_token(result).map_err(convert_to_rejection)?;
                let cookie = format!(
                    // "jwt={}; Path=/; HttpOnly; Max-Age=1209600; Secure; SameSite=None; Domain=http://192.168.1.115:3000",
                    // "jwt={}; Path=/; HttpOnly; Max-Age=1209600; Secure; SameSite=Lax;",

                    // the below jwt works in dev server + HTTP
                    "jwt={}; Path=/; HttpOnly; Max-Age=1209600; SameSite=Lax;",
                    token
                );

                let json_resp =
                    warp::reply::json(&json!({ "message": format!("login success!",) }));

                return Ok(warp::reply::with_header(json_resp, SET_COOKIE, cookie));
            } else {
                return Err(Error::invalid_usr_pwd("invalid user or password").into());
            }
        }
        // user not fund
        Err(DieselError::NotFound) => {
            return Err(Error::invalid_usr_pwd("invalid user or password").into());
        }
        Err(_) => return Err(Error::database("internal server error!").into()),
    }
}

/// This function creates a normal user, to create admins, use admin_create
pub async fn user_create(rcvd_payload: User, conn: DbConnection) -> Result<impl Reply, Rejection> {
    use crate::schema::users::dsl::*;
    if !valid_pwd(&rcvd_payload.user_pwd) {
        return Err(convert_to_rejection(Error::invalid_usr_pwd(
            "password must be at least 8 characters long!",
        )));
    }

    let mut conn = conn.map_err(convert_to_rejection)?;
    let query = diesel::insert_into(users)
        .values::<User>(User {
            id: None,
            user_name: rcvd_payload.user_name.clone(),

            user_role: Some("usr".into()),
            user_pwd: hash(rcvd_payload.user_pwd, 4).map_err(convert_to_rejection)?,
        })
        .execute(&mut conn);

    match query {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "message": format!("User {} created!", rcvd_payload.user_name)
            })),
            StatusCode::CREATED,
        )),
        Err(error_kind) => match error_kind {
            // enforcing uniqueness of "user_name" field
            DieselError::DatabaseError(db_err_kind, msg) => match db_err_kind {
                DatabaseErrorKind::UniqueViolation => {
                    Err(Error::unique_violation(rcvd_payload.user_name).into())
                }
                _ => Err(Error::database(msg.message()).into()),
            },
            _ => Err(Error::database("internal server error! ").into()),
        },
    }
}

pub async fn admin_create(rcvd_payload: User, conn: DbConnection) -> Result<impl Reply, Rejection> {
    use crate::schema::users::dsl::*;
    if !valid_pwd(&rcvd_payload.user_pwd) {
        return Err(convert_to_rejection(Error::invalid_usr_pwd(
            "password must be at least 8 characters long!",
        )));
    }

    let mut conn = conn.map_err(convert_to_rejection)?;
    let query = diesel::insert_into(users)
        .values::<User>(User {
            id: None,
            user_name: rcvd_payload.user_name.clone(),

            user_role: Some("adm".into()),
            user_pwd: hash(rcvd_payload.user_pwd, 4).map_err(convert_to_rejection)?,
        })
        .execute(&mut conn);
    match query {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "message": format!("Admin {} created!", rcvd_payload.user_name)
            })),
            StatusCode::CREATED,
        )),
        Err(error_kind) => match error_kind {
            // enforcing uniqueness of "user_name" field
            DieselError::DatabaseError(db_err_kind, msg) => match db_err_kind {
                DatabaseErrorKind::UniqueViolation => {
                    Err(Error::unique_violation(rcvd_payload.user_name).into())
                }
                _ => Err(Error::database(msg.message()).into()),
            },
            _ => Err(Error::database("internal server error! ").into()),
        },
    }
}

fn valid_pwd(pwd: &String) -> bool {
    // TODO: implement pwd validation here
    if pwd.len() <= 8 {
        return false;
    } else {
        return true;
    }
}
