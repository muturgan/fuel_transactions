mod implementations;
pub mod models;

use crate::dto::{ApiTransaction, UserId};
use crate::system_models::AppError;
use crate::{config, dto::TxId};
use implementations::{MockStore, PostgresStore};
use models::Transaction;

#[derive(Clone)]
enum StoreKind {
	Mock(MockStore),
	Postgres(PostgresStore),
}

trait Store {
	async fn get_transactions_list(&self) -> Result<Vec<Transaction>, AppError>;

	async fn get_transaction(&self, tx_id: TxId) -> Result<Transaction, AppError>;

	async fn create_transaction(
		&self,
		user_id: UserId,
		new_tx: ApiTransaction,
	) -> Result<Transaction, AppError>;

	async fn update_transaction(
		&self,
		tx_id: TxId,
		user_id: UserId,
		tx: ApiTransaction,
	) -> Result<Transaction, AppError>;

	async fn delete_transaction(&self, tx_id: TxId, user_id: UserId) -> Result<(), AppError>;

	async fn close(&self);
}

#[derive(Clone)]
pub struct Repository {
	store: StoreKind,
}

impl Repository {
	pub async fn new() -> Self {
		if config::is_test() {
			return Self {
				store: StoreKind::Mock(MockStore::new()),
			};
		}

		return Self {
			store: StoreKind::Postgres(PostgresStore::new().await),
		};
	}

	pub async fn get_transactions_list(&self) -> Result<Vec<Transaction>, AppError> {
		match &self.store {
			StoreKind::Mock(store) => store.get_transactions_list().await,
			StoreKind::Postgres(store) => store.get_transactions_list().await,
		}
	}

	pub async fn get_transaction(&self, tx_id: TxId) -> Result<Transaction, AppError> {
		match &self.store {
			StoreKind::Mock(store) => store.get_transaction(tx_id).await,
			StoreKind::Postgres(store) => store.get_transaction(tx_id).await,
		}
	}

	pub async fn create_transaction(
		&self,
		user_id: UserId,
		new_tx: ApiTransaction,
	) -> Result<Transaction, AppError> {
		match &self.store {
			StoreKind::Mock(store) => store.create_transaction(user_id, new_tx).await,
			StoreKind::Postgres(store) => store.create_transaction(user_id, new_tx).await,
		}
	}

	pub async fn update_transaction(
		&self,
		tx_id: TxId,
		user_id: UserId,
		tx: ApiTransaction,
	) -> Result<Transaction, AppError> {
		match &self.store {
			StoreKind::Mock(store) => store.update_transaction(tx_id, user_id, tx).await,
			StoreKind::Postgres(store) => store.update_transaction(tx_id, user_id, tx).await,
		}
	}

	pub async fn delete_transaction(&self, tx_id: TxId, user_id: UserId) -> Result<(), AppError> {
		match &self.store {
			StoreKind::Mock(store) => store.delete_transaction(tx_id, user_id).await,
			StoreKind::Postgres(store) => store.delete_transaction(tx_id, user_id).await,
		}
	}

	pub async fn close(&self) {
		match &self.store {
			StoreKind::Mock(store) => store.close().await,
			StoreKind::Postgres(store) => store.close().await,
		};
	}
}
