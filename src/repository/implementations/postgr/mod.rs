mod pool;

use super::super::Store;
use crate::dto::{TxId, UserId};
use crate::repository::models::Transaction;
use crate::{dto::ApiTransaction, system_models::AppError};
use sqlx::{Error as EqlxError, PgPool};

impl From<EqlxError> for AppError {
	fn from(err: EqlxError) -> Self {
		return AppError::SystemError(err.to_string());
	}
}

#[derive(Clone)]
pub struct PostgresStore {
	pool: PgPool,
}

impl PostgresStore {
	pub async fn new() -> Self {
		let pool = pool::create_db_connection().await;
		Self { pool }
	}
}

impl Store for PostgresStore {
	async fn get_transactions_list(&self) -> Result<Vec<Transaction>, AppError> {
		let txs =
			sqlx::query_as::<_, Transaction>("SELECT * FROM transactions ORDER BY date_created ASC;")
				.fetch_all(&self.pool)
				.await?;

		return Ok(txs);
	}

	async fn get_transaction(&self, TxId(tx_id): TxId) -> Result<Transaction, AppError> {
		let mut rows = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1;")
			.bind(tx_id)
			.fetch_all(&self.pool)
			.await?;

		return match rows.pop() {
			None => Err(AppError::NotFound(format!(
				"Transaction with id {tx_id} not found"
			))),
			Some(tx) => Ok(tx),
		};
	}

	async fn create_transaction(
		&self,
		UserId(user_id): UserId,
		new_tx: ApiTransaction,
	) -> Result<Transaction, AppError> {
		let inserted_tx = sqlx::query_as::<_, Transaction>(
			"INSERT INTO transactions (
				op_date,
				gas_station_id,
				card_id,
				contract_id,
				nomenclature_id,
				amount,
				stella_sum,
				stella_nds_sum,
				refund,
				buy_sum_plan,
				buy_nds_sum_plan,
				buy_sum_fact,
				buy_nds_sum_fact,
				sell_sum_plan,
				sell_nds_sum_plan,
				sell_sum_fact,
				sell_nds_sum_fact,
				implementation_id,
				user_id
			) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
				$11, $12, $13, $14, $15, $16, $17, $18, $19)
			RETURNING *;",
		)
		.bind(new_tx.op_date)
		.bind(new_tx.gas_station_id)
		.bind(new_tx.card_id)
		.bind(new_tx.contract_id)
		.bind(new_tx.nomenclature_id)
		.bind(new_tx.amount)
		.bind(new_tx.stella_sum)
		.bind(new_tx.stella_nds_sum)
		.bind(new_tx.refund)
		.bind(new_tx.buy_sum_plan)
		.bind(new_tx.buy_nds_sum_plan)
		.bind(new_tx.buy_sum_fact)
		.bind(new_tx.buy_nds_sum_fact)
		.bind(new_tx.sell_sum_plan)
		.bind(new_tx.sell_nds_sum_plan)
		.bind(new_tx.sell_sum_fact)
		.bind(new_tx.sell_nds_sum_fact)
		.bind(new_tx.implementation_id)
		.bind(user_id)
		.fetch_one(&self.pool)
		.await?;

		return Ok(inserted_tx);
	}

	async fn update_transaction(
		&self,
		TxId(tx_id): TxId,
		UserId(user_id): UserId,
		tx: ApiTransaction,
	) -> Result<Transaction, AppError> {
		let mut rows = sqlx::query_as::<_, Transaction>(
			"UPDATE transactions
			SET op_date = $1,
				gas_station_id = $2,
				card_id = $3,
				contract_id = $4,
				nomenclature_id = $5,
				amount = $6,
				stella_sum = $7,
				stella_nds_sum = $8,
				refund = $9,
				buy_sum_plan = $10,
				buy_nds_sum_plan = $11,
				buy_sum_fact = $12,
				buy_nds_sum_fact = $13,
				sell_sum_plan = $14,
				sell_nds_sum_plan = $15,
				sell_sum_fact = $16,
				sell_nds_sum_fact = $17,
				implementation_id = $18,
				user_id = $19
			WHERE id = $20
			RETURNING *;",
		)
		.bind(tx.op_date)
		.bind(tx.gas_station_id)
		.bind(tx.card_id)
		.bind(tx.contract_id)
		.bind(tx.nomenclature_id)
		.bind(tx.amount)
		.bind(tx.stella_sum)
		.bind(tx.stella_nds_sum)
		.bind(tx.refund)
		.bind(tx.buy_sum_plan)
		.bind(tx.buy_nds_sum_plan)
		.bind(tx.buy_sum_fact)
		.bind(tx.buy_nds_sum_fact)
		.bind(tx.sell_sum_plan)
		.bind(tx.sell_nds_sum_plan)
		.bind(tx.sell_sum_fact)
		.bind(tx.sell_nds_sum_fact)
		.bind(tx.implementation_id)
		.bind(user_id)
		.bind(tx_id)
		.fetch_all(&self.pool)
		.await?;

		return match rows.pop() {
			None => Err(AppError::NotFound(format!(
				"Transaction with id {tx_id} not found"
			))),
			Some(tx) => Ok(tx),
		};
	}

	async fn delete_transaction(
		&self,
		TxId(tx_id): TxId,
		UserId(user_id): UserId,
	) -> Result<(), AppError> {
		let mut rows = sqlx::query_as::<_, Transaction>(
			"UPDATE transactions
			SET deleted = true,
				user_id = $1
			WHERE id = $2
			RETURNING *;",
		)
		.bind(user_id)
		.bind(tx_id)
		.fetch_all(&self.pool)
		.await?;

		return match rows.pop() {
			None => Err(AppError::NotFound(format!(
				"Transaction with id {tx_id} not found"
			))),
			Some(_) => Ok(()),
		};
	}

	async fn close(&self) {
		self.pool.close().await;
	}
}
