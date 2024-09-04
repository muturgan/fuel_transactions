use ::std::error::Error;

use axum::{
	async_trait,
	extract::{rejection::JsonRejection, FromRequest, Request},
	http::{HeaderMap, Uri},
	Json, RequestExt,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::system_models::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiTransaction {
	pub op_date: DateTime<Utc>,
	pub gas_station_id: Uuid,
	pub card_id: Option<Uuid>,
	pub contract_id: Option<Uuid>,
	pub nomenclature_id: Uuid,
	pub amount: Option<f64>,
	pub stella_sum: Option<f64>,
	pub stella_nds_sum: Option<f64>,
	pub refund: bool,
	pub buy_sum_plan: Option<f64>,
	pub buy_nds_sum_plan: Option<f64>,
	pub buy_sum_fact: Option<f64>,
	pub buy_nds_sum_fact: Option<f64>,
	pub sell_sum_plan: Option<f64>,
	pub sell_nds_sum_plan: Option<f64>,
	pub sell_sum_fact: Option<f64>,
	pub sell_nds_sum_fact: Option<f64>,
	pub implementation_id: Option<Uuid>,
}

#[async_trait]
impl<S> FromRequest<S> for ApiTransaction {
	type Rejection = AppError;

	async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
		let body = req.extract::<Json<ApiTransaction>, _>().await;

		return match body {
			Err(err) => Err(match err {
				JsonRejection::JsonDataError(data_err) => match data_err.source() {
					Some(source_err) => {
						AppError::BadRequest(format!("Передано некорректное тело запроса: {source_err}"))
					}
					None => AppError::BadRequest(String::from("Передано некорректное тело запроса")),
				},

				JsonRejection::JsonSyntaxError(_) => {
					AppError::BadRequest(String::from("Передано некорректное тело запроса"))
				}

				JsonRejection::MissingJsonContentType(_) => AppError::BadRequest(String::from(
					"Пожалуйста, укажите заголовок `Content-Type: application/json`",
				)),

				JsonRejection::BytesRejection(_) => {
					AppError::SystemError(String::from("Не удалось прочитать тело запроса"))
				}

				non_exhaustive => AppError::SystemError(non_exhaustive.to_string()),
			}),
			Ok(Json(dto)) => Ok(dto),
		};
	}
}

pub struct TxId(pub Uuid);

impl TxId {
	pub fn from_uri(uri: &Uri) -> Result<Self, AppError> {
		let id_param = uri.to_string().split("/").map(|s| s.to_owned()).last();

		if id_param.is_none() {
			return Err(AppError::BadRequest(String::from(
				"Некорректный путь запроса",
			)));
		}

		let id_param = id_param.unwrap();

		let tx_id = Uuid::parse_str(&id_param);

		if tx_id.is_err() {
			return Err(AppError::BadRequest(String::from(
				"Некорректное значение идентификатора транзакции",
			)));
		}

		return Ok(TxId(tx_id.unwrap()));
	}
}

#[async_trait]
impl<S> FromRequest<S> for TxId {
	type Rejection = AppError;

	async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
		return TxId::from_uri(req.uri());
	}
}

pub struct UserId(pub Uuid);

impl UserId {
	pub fn from_headers(headers: &HeaderMap) -> Result<Self, AppError> {
		let user_id_header = headers.get("X-USER-ID");

		if user_id_header.is_none() {
			return Err(AppError::BadRequest(String::from(
				"Не передан заголовок X-USER-ID",
			)));
		}

		let user_id_header = user_id_header.unwrap().to_str();

		if user_id_header.is_err() {
			return Err(AppError::BadRequest(String::from(
				"Некорректное значение заголовка X-USER-ID",
			)));
		}

		let user_id_header = user_id_header.unwrap();

		let user_id = Uuid::parse_str(user_id_header);

		if user_id.is_err() {
			return Err(AppError::BadRequest(String::from(
				"Некорректное значение заголовка X-USER-ID",
			)));
		}

		return Ok(UserId(user_id.unwrap()));
	}
}
