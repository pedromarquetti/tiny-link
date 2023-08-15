use diesel::{
    r2d2::Error as R2D2Error,
    result::{DatabaseErrorKind, Error as DieselError},
};

use hyper::{header::InvalidHeaderValue, http::uri::InvalidUri};
use serde_json::json;
use std::convert::Infallible;
use url::ParseError;
use warp::{
    body::BodyDeserializeError,
    http::StatusCode,
    reject::{self, MethodNotAllowed, Rejection},
    reply::{self, Reply, WithStatus},
};

/// Error that can be converted to API error
pub fn convert_to_rejection<T: Into<Error>>(error: T) -> Rejection {
    reject::custom(error.into())
}

/// Returns error Response based on suplied parameters
pub async fn handle_rejection(err: Rejection) -> Result<WithStatus<Box<dyn Reply>>, Infallible> {
    // if err.find has 'e' as Err
    if let Some(e) = err.find::<Error>() {
        error!("{:?}", e);
        // creates json reply
        Ok(e.to_json_reply())
    } else if let Some(err) = err.find::<BodyDeserializeError>() {
        error!("{}", err);
        Ok(Error::custom(err.to_string(), StatusCode::BAD_REQUEST).to_json_reply())
    } else if let Some(err) = err.find::<MethodNotAllowed>() {
        error!("{}", err);
        Ok(Error::custom(err.to_string(), StatusCode::BAD_REQUEST).to_json_reply())
    } else {
        // something else happened
        Ok(Error::custom_with_log(
            format!("{:#?}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{:?}", err),
        )
        .to_json_reply())
    }
}

#[derive(Debug)]
/// Types of error that can occur
enum ErrorKind {
    InvalidUsrPwd,

    Database,
    InvalidForm,
    Custom(String),
    UniqueViolation,
}

#[derive(Debug)]
/// Struct that represents possible Errors
///
/// to_json_reply() creates JSON responses to reply to the server
///
/// # Usage
///
/// 1.Error::(insert function here).to_json_reply()
pub struct Error {
    kind: ErrorKind,
    status_code: StatusCode,
    msg: Option<String>,
}

impl Error {
    /// Errors that can occur with user login/register
    pub fn user_validation_err<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::InvalidUsrPwd,
            status_code: StatusCode::UNAUTHORIZED,
            msg: Some(msg.into()),
        }
    }
    /// Invalid received POST request forms!
    pub fn invalid_forms<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::InvalidForm,
            status_code: StatusCode::BAD_REQUEST,
            msg: Some(msg.into()),
        }
    }
    /// Database Error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self {
            kind: ErrorKind::Database,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: Some(message.into()),
        }
    }

    pub fn unique_violation<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::UniqueViolation,
            status_code: StatusCode::CONFLICT,
            msg: Some(msg.into()),
        }
    }

    pub fn custom<S: Into<String>>(msg: S, status_code: StatusCode) -> Self {
        Self {
            kind: ErrorKind::Custom(msg.into()),
            status_code,
            msg: None,
        }
    }

    pub fn custom_with_log<S: Into<String>>(
        msg: S,
        status_code: StatusCode,
        log_msg: String,
    ) -> Self {
        Self {
            kind: ErrorKind::Custom(msg.into()),
            status_code,
            msg: Some(log_msg),
        }
    }

    /// Converts Error to json
    fn to_json_reply(&self) -> WithStatus<Box<dyn Reply>> {
        if let Some(log) = &self.msg {
            // log error to console
            error!("{}", log);
        }

        let curr_msg = &self.msg;

        // response body
        let body: Box<dyn Reply> = match &self.kind {
            ErrorKind::UniqueViolation => Box::new(reply::json(&json!({ "error": curr_msg }))),
            ErrorKind::InvalidUsrPwd => Box::new(reply::json(&json!({ "error": curr_msg }))),
            ErrorKind::Database => Box::new(reply::json(&json!({ "error": curr_msg }))),
            ErrorKind::InvalidForm => Box::new(reply::json(&json!({ "error": curr_msg }))),

            ErrorKind::Custom(msg) => Box::new(reply::json(&json!({ "error": msg }))),
        };

        // return response
        reply::with_status(body, self.status_code)
    }
}

impl reject::Reject for Error {}
impl From<DieselError> for Error {
    fn from(diesel_err: DieselError) -> Self {
        match diesel_err {
            DieselError::DatabaseError(kind, error_msg) => match kind {
                DatabaseErrorKind::UniqueViolation => Error::unique_violation(error_msg.message()),
                _ => Error::database(error_msg.message()),
            },
            err => Error::database(err.to_string()),
        }
    }
}
impl From<R2D2Error> for Error {
    fn from(value: R2D2Error) -> Self {
        Error::database(value.to_string())
    }
}

impl From<r2d2::Error> for Error {
    fn from(value: r2d2::Error) -> Self {
        Error::database(value.to_string())
    }
}

/// Error while trying to parse Uri, this is a invalid form
impl From<ParseError> for Error {
    fn from(msg: ParseError) -> Self {
        Error::invalid_forms(msg.to_string())
    }
}

/// Error while trying to parse Uri, this is a invalid form
impl From<InvalidUri> for Error {
    fn from(msg: InvalidUri) -> Self {
        Error::invalid_forms(msg.to_string())
    }
}
impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        Error::user_validation_err(value.to_string())
    }
}
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Error::custom(
            format!("JWT Error! {}", value.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}
impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Error::custom(
            format!("INTERNAL SERVER ERROR! {}", value.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}
