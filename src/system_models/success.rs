use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::Serialize;

pub struct Success<T: Serialize>(pub StatusCode, pub T);

impl<T: Serialize> IntoResponse for Success<T> {
	fn into_response(self) -> Response {
		(self.0, Json(self.1)).into_response()
	}
}
