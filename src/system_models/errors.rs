use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FmtResult};
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
	BadRequest(String),
	NotFound(String),
	SystemError(String),
}

impl Display for AppError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		return match self {
			AppError::BadRequest(msg) => {
				write!(f, "BadRequest: {msg}")
			}
			AppError::NotFound(msg) => {
				write!(f, "NotFound: {msg}")
			}
			AppError::SystemError(msg) => {
				write!(f, "SystemError: {msg}")
			}
		};
	}
}

impl Error for AppError {}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		match self {
			AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
			AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
			AppError::SystemError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
		}
	}
}
