use crate::handler as H;
use crate::repository::Repository;
use ::std::sync::Arc;
use axum::{routing::get, Router};

pub fn create_router(repo: Arc<Repository>) -> Router {
	return Router::new()
		.route(
			"/api/v1/transactions",
			get(H::get_transactions_list).post(H::create_transaction),
		)
		.route(
			"/api/v1/transactions/:id",
			get(H::get_transaction)
				.put(H::update_transaction)
				.delete(H::delete_transaction),
		)
		.with_state(repo);
}
