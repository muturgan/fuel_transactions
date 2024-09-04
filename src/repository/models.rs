use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct Transaction {
	pub id: Uuid,
	pub op_date: DateTime<Utc>,
	pub gas_station_id: Uuid,
	pub card_id: Option<Uuid>,
	pub contract_id: Option<Uuid>,
	pub nomenclature_id: Uuid,
	pub amount: Option<Decimal>,
	pub stella_sum: Option<Decimal>,
	pub stella_nds_sum: Option<Decimal>,
	pub refund: bool,
	pub buy_sum_plan: Option<Decimal>,
	pub buy_nds_sum_plan: Option<Decimal>,
	pub buy_sum_fact: Option<Decimal>,
	pub buy_nds_sum_fact: Option<Decimal>,
	pub sell_sum_plan: Option<Decimal>,
	pub sell_nds_sum_plan: Option<Decimal>,
	pub sell_sum_fact: Option<Decimal>,
	pub sell_nds_sum_fact: Option<Decimal>,
	pub implementation_id: Option<Uuid>,
	pub user_id: Uuid,
	pub date_created: DateTime<Utc>,
	pub date_updated: Option<DateTime<Utc>>,
	pub deleted: bool,
}
