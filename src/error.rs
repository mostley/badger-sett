use std::borrow::Cow;

use derive_more::From;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder, Response};
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::broadcast;
use rocket::Request;
use sqlx::sqlite::SqliteError;
use sqlx::SqliteExecutor;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    BadRequest(String),

    DataConflict(String),

    #[from]
    DBError(sqlx::Error),
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }

    fn to_status(&self) -> Status {
        match self {
            Error::BadRequest(_) => Status::BadRequest,
            Error::DataConflict(_) => Status::Conflict,

            Error::DBError(db_error) => match db_error {
                sqlx::Error::RowNotFound => Status::NotFound,
                err => {
                    if let Some(sqlite_error) = err.as_database_error() {
                        match sqlite_error.code() {
                            Some(code) if code == "2067" => Status::Conflict,
                            _ => Status::InternalServerError,
                        }
                    } else {
                        Status::InternalServerError
                    }
                }
            },

            _ => Status::InternalServerError,
        }
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        let error = match self {
            Error::Custom(err) => err.to_owned(),
            Error::BadRequest(err) => err.to_owned(),
            Error::DataConflict(err) => err.to_owned(),

            Error::DBError(db_error) => match db_error {
                sqlx::Error::RowNotFound => "Entity not found".into(),
                err => {
                    print!("{err:?}");

                    if let Some(sqlite_error) = err.as_database_error() {
                        match sqlite_error.code() {
                            Some(code) if code == "2067" => sqlite_error.message().into(),
                            _ => "DatbaseRequest failed".into(),
                        }
                    } else {
                        "Database request failed".into()
                    }
                }
            },
        };

        return fmt.write_str(&error);
    }
}

impl std::error::Error for Error {}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let response = Json(ErrorResponse {
            error: self.to_string(),
        });
        Response::build_from(response.respond_to(request)?)
            .status(self.to_status())
            .header(ContentType::JSON)
            .ok()
    }
}
