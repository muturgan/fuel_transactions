use crate::{
	dto::{ApiTransaction, TxId, UserId},
	repository::{models::Transaction, Repository},
	system_models::{AppError, Success},
};
use ::std::sync::Arc;
use axum::{
	extract::{FromRequest, Request, State},
	http::StatusCode,
};

#[utoipa::path(
	get,
	path = "/api/v1/transactions",
	responses(
		(status = 200, description = "Returns a list of transactions", body = [Transaction])
	)
)]
pub async fn get_transactions_list(
	State(repo): State<Arc<Repository>>,
) -> Result<Success<Vec<Transaction>>, AppError> {
	let list = repo.get_transactions_list().await?;
	return Ok(Success(StatusCode::OK, list));
}

#[utoipa::path(
	get,
	path = "/api/v1/transactions/{tx_id}",
	params(
		("tx_id" = Uuid, Path, description = "transaction id")
	),
	responses(
		(status = 200, description = "Returns a transaction by id", body = Transaction),
		(status = 404),
		(status = 500)
	),
)]
pub async fn get_transaction(
	State(repo): State<Arc<Repository>>,
	tx_id: TxId,
) -> Result<Success<Transaction>, AppError> {
	let tx = repo.get_transaction(tx_id).await?;
	return Ok(Success(StatusCode::OK, tx));
}

#[utoipa::path(
	post,
	path = "/api/v1/transactions",
	params(
		("X-USER-ID" = Uuid, Header, description = "Current user id"),
	),
	request_body(content = ApiTransaction, content_type = "application/json"),
	responses(
		(status = 201, description = "Create a new transaction", body = Transaction),
		(status = 400),
		(status = 500)
	)
)]
pub async fn create_transaction(
	State(repo): State<Arc<Repository>>,
	req: Request,
) -> Result<Success<Transaction>, AppError> {
	let user_id = UserId::from_headers(req.headers())?;
	let new_tx = ApiTransaction::from_request(req, &()).await?;

	let inserted_tx = repo.create_transaction(user_id, new_tx).await?;
	return Ok(Success(StatusCode::CREATED, inserted_tx));
}

#[utoipa::path(
	put,
	path = "/api/v1/transactions/{tx_id}",
	params(
		("tx_id" = Uuid, Path, description = "transaction id"),
		("X-USER-ID" = Uuid, Header, description = "Current user id"),
	),
	request_body(content = ApiTransaction, content_type = "application/json"),
	responses(
		(status = 202, description = "Update a transaction by id", body = Transaction),
		(status = 400),
		(status = 404),
		(status = 500)
	),
)]
pub async fn update_transaction(
	State(repo): State<Arc<Repository>>,
	req: Request,
) -> Result<Success<Transaction>, AppError> {
	let tx_id = TxId::from_uri(req.uri())?;
	let user_id = UserId::from_headers(req.headers())?;
	let tx = ApiTransaction::from_request(req, &()).await?;

	let updated_tx = repo.update_transaction(tx_id, user_id, tx).await?;
	return Ok(Success(StatusCode::ACCEPTED, updated_tx));
}

#[utoipa::path(
	delete,
	path = "/api/v1/transactions/{tx_id}",
	responses(
		(status = 204, description = "Delete a transaction by id", body = ()),
		(status = 404),
		(status = 500)
	),
	params(
		("tx_id" = Uuid, Path, description = "transaction id"),
		("X-USER-ID" = Uuid, Header, description = "Current user id"),
	),
)]
pub async fn delete_transaction(
	State(repo): State<Arc<Repository>>,
	req: Request,
) -> Result<StatusCode, AppError> {
	let tx_id = TxId::from_uri(req.uri())?;
	let user_id = UserId::from_headers(req.headers())?;

	repo.delete_transaction(tx_id, user_id).await?;
	return Ok(StatusCode::NO_CONTENT);
}
