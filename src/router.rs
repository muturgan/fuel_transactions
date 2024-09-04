use crate::{
	dto::ApiTransaction,
	handler as H,
	repository::{models::Transaction, Repository},
};
use ::std::sync::Arc;
use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "fuel", description = "a CRUD service to work with transactions of fuel issuers"),
	),
	paths(H::get_transactions_list, H::get_transaction, H::create_transaction, H::update_transaction, H::delete_transaction,),
	components(schemas(ApiTransaction, Transaction))
)]
struct ApiDoc;

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
		.with_state(repo)
		.merge(SwaggerUi::new("/swagger").url("/swagger/swagger.json", ApiDoc::openapi()));
}
