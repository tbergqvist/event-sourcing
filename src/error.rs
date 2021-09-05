use std::{fmt::{self, Display}};

#[derive(Debug)]
pub enum Error {
	Other(String),
  Database(mysql::Error),
  Json(serde_json::Error),
}

impl Display for Error {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Other(err) => write!(formatter, "error: {}", err),
			Self::Database(inner_err) => write!(formatter, "DatabaseError: {}", inner_err),
			Self::Json(inner_err) => write!(formatter, "JsonError: {}", inner_err),
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<mysql::Error> for Error {
	fn from(error: mysql::Error) -> Self {
		Error::Database(error)
	}
}

impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Self {
		Error::Json(error)
	}
}