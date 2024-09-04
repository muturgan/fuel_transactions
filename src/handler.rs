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

pub async fn get_transactions_list(
	State(repo): State<Arc<Repository>>,
) -> Result<Success<Vec<Transaction>>, AppError> {
	let list = repo.get_transactions_list().await?;
	return Ok(Success(StatusCode::OK, list));
}

pub async fn get_transaction(
	State(repo): State<Arc<Repository>>,
	tx_id: TxId,
) -> Result<Success<Transaction>, AppError> {
	let tx = repo.get_transaction(tx_id).await?;
	return Ok(Success(StatusCode::OK, tx));
}

pub async fn create_transaction(
	State(repo): State<Arc<Repository>>,
	req: Request,
) -> Result<Success<Transaction>, AppError> {
	let user_id = UserId::from_headers(req.headers())?;
	let new_tx = ApiTransaction::from_request(req, &()).await?;

	let inserted_tx = repo.create_transaction(user_id, new_tx).await?;
	return Ok(Success(StatusCode::CREATED, inserted_tx));
}

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

pub async fn delete_transaction(
	State(repo): State<Arc<Repository>>,
	req: Request,
) -> Result<StatusCode, AppError> {
	let tx_id = TxId::from_uri(req.uri())?;
	let user_id = UserId::from_headers(req.headers())?;

	repo.delete_transaction(tx_id, user_id).await?;
	return Ok(StatusCode::NO_CONTENT);
}
