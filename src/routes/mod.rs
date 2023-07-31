mod api;
mod auth;
mod ui;
mod user;

use auth::auth;
use serde_json::json;
use warp::{path, Filter, Rejection, Reply};

use crate::db::Pool;

async fn ping() -> Result<impl Reply, Rejection> {
    return Ok(warp::reply::json(&json!({"ping":"success!"})));
}
/// Routing table for API
pub fn builder(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.get());

    let ping = warp::any()
        .and(path("api"))
        .and(path("ping"))
        .and(path::end())
        .and_then(ping);

    // user endpoints
    let handle_login = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path("api"))
        .and(path("user"))
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(user::user_login);

    let create_admin = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path("api"))
        .and(path("admin"))
        .and(path("create"))
        .and(path::end())
        .and(auth())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(user::admin_create);

    let create_user = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path("api"))
        .and(path("user"))
        .and(path("create"))
        .and(path::end())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(user::user_create);

    // use path to redirect to corresponding url
    let redirect_link = warp::get()
        .and(warp::path::param())
        // the server will only accept non empty paths
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(api::redirect_to_link);

    // create new link
    let create_link = warp::post()
        .and(path("api"))
        .and(path("link"))
        .and(path("create"))
        .and(path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(api::create_link);

    // ui endpoints
    let serve_index = warp::get().and(warp::path::end()).and_then(ui::serve_index);

    let link_endpoints = redirect_link.or(create_link);
    let protected_endpoints = create_admin;

    let routes = handle_login
        .or(protected_endpoints)
        .or(create_user)
        .or(link_endpoints)
        .or(serve_index)
        .or(ping);

    routes.clone()
}
