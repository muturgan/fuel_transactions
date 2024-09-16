use super::super::Store;
use crate::dto::{ApiTransaction, TxId, UserId};
use crate::repository::models::Transaction;
use crate::system_models::AppError;
use ::std::sync::Arc;
use chrono::Utc;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct MockStore {
	store: Arc<RwLock<Vec<Transaction>>>,
}

impl MockStore {
	pub fn new() -> Self {
		Self {
			store: Arc::new(RwLock::new(Vec::new())),
		}
	}
}

impl Store for MockStore {
	async fn get_transactions_list(&self) -> Result<Vec<Transaction>, AppError> {
		let current_store = self.store.read().await;
		return Ok(current_store.iter().cloned().collect());
	}

	async fn get_transaction(&self, TxId(tx_id): TxId) -> Result<Transaction, AppError> {
		let current_store = self.store.read().await;
		let entry = current_store.iter().find(|tx| tx.id == tx_id);

		return match entry {
			None => Err(AppError::NotFound(format!(
				"Transaction with id {tx_id} not found"
			))),
			Some(tx) => Ok(tx.clone()),
		};
	}

	async fn create_transaction(
		&self,
		UserId(user_id): UserId,
		new_tx: ApiTransaction,
	) -> Result<Transaction, AppError> {
		let now = Utc::now();

		let tx = Transaction {
			id: Uuid::new_v4(),
			op_date: new_tx.op_date,
			gas_station_id: new_tx.gas_station_id,
			card_id: new_tx.card_id,
			contract_id: new_tx.contract_id,
			nomenclature_id: new_tx.nomenclature_id,
			amount: new_tx.amount.map(|n| Decimal::from_f64(n).unwrap()),
			stella_sum: new_tx.stella_sum.map(|n| Decimal::from_f64(n).unwrap()),
			stella_nds_sum: new_tx.stella_nds_sum.map(|n| Decimal::from_f64(n).unwrap()),
			refund: new_tx.refund,
			buy_sum_plan: new_tx.buy_sum_plan.map(|n| Decimal::from_f64(n).unwrap()),
			buy_nds_sum_plan: new_tx
				.buy_nds_sum_plan
				.map(|n| Decimal::from_f64(n).unwrap()),
			buy_sum_fact: new_tx.buy_sum_fact.map(|n| Decimal::from_f64(n).unwrap()),
			buy_nds_sum_fact: new_tx
				.buy_nds_sum_fact
				.map(|n| Decimal::from_f64(n).unwrap()),
			sell_sum_plan: new_tx.sell_sum_plan.map(|n| Decimal::from_f64(n).unwrap()),
			sell_nds_sum_plan: new_tx
				.sell_nds_sum_plan
				.map(|n| Decimal::from_f64(n).unwrap()),
			sell_sum_fact: new_tx.sell_sum_fact.map(|n| Decimal::from_f64(n).unwrap()),
			sell_nds_sum_fact: new_tx
				.sell_nds_sum_fact
				.map(|n| Decimal::from_f64(n).unwrap()),
			implementation_id: new_tx.implementation_id,
			user_id,
			date_created: now,
			date_updated: Some(now),
			deleted: false,
		};

		let mut current_store = self.store.write().await;
		current_store.push(tx.clone());

		return Ok(tx);
	}

	async fn update_transaction(
		&self,
		TxId(tx_id): TxId,
		UserId(user_id): UserId,
		tx: ApiTransaction,
	) -> Result<Transaction, AppError> {
		let mut current_store = self.store.write().await;

		let existing_tx = current_store.iter_mut().find(|t| t.id == tx_id);

		if existing_tx.is_none() {
			return Err(AppError::NotFound(format!(
				"Transaction with id {tx_id} not found"
			)));
		}

		let existing_tx = existing_tx.unwrap();
		existing_tx.op_date = tx.op_date;
		existing_tx.gas_station_id = tx.gas_station_id;
		existing_tx.card_id = tx.card_id;
		existing_tx.contract_id = tx.contract_id;
		existing_tx.nomenclature_id = tx.nomenclature_id;
		existing_tx.amount = tx.amount.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.stella_sum = tx.stella_sum.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.stella_nds_sum = tx.stella_nds_sum.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.refund = tx.refund;
		existing_tx.buy_sum_plan = tx.buy_sum_plan.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.buy_nds_sum_plan = tx.buy_nds_sum_plan.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.buy_sum_fact = tx.buy_sum_fact.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.buy_nds_sum_fact = tx.buy_nds_sum_fact.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.sell_sum_plan = tx.sell_sum_plan.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.sell_nds_sum_plan = tx.sell_nds_sum_plan.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.sell_sum_fact = tx.sell_sum_fact.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.sell_nds_sum_fact = tx.sell_nds_sum_fact.map(|n| Decimal::from_f64(n).unwrap());
		existing_tx.implementation_id = tx.implementation_id;
		existing_tx.user_id = user_id;
		existing_tx.date_updated = Some(Utc::now());

		return Ok(existing_tx.clone());
	}

	async fn delete_transaction(
		&self,
		TxId(tx_id): TxId,
		UserId(user_id): UserId,
	) -> Result<(), AppError> {
		let mut current_store = self.store.write().await;

		let existing_tx = current_store.iter_mut().find(|t| t.id == tx_id);

		if existing_tx.is_none() {
			return Err(AppError::NotFound(format!(
				"Transaction with id {tx_id} not found"
			)));
		}

		let existing_tx = existing_tx.unwrap();
		existing_tx.deleted = true;
		existing_tx.user_id = user_id;
		existing_tx.date_updated = Some(Utc::now());

		return Ok(());
	}

	async fn close(&self) {}
}
