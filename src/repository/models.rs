use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Transaction {
	pub id: Uuid,
	pub op_date: DateTime<Utc>,
	pub gas_station_id: Uuid,
	pub card_id: Option<Uuid>,
	pub contract_id: Option<Uuid>,
	pub nomenclature_id: Uuid,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub amount: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub stella_sum: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub stella_nds_sum: Option<Decimal>,

	pub refund: bool,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub buy_sum_plan: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub buy_nds_sum_plan: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub buy_sum_fact: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub buy_nds_sum_fact: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub sell_sum_plan: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub sell_nds_sum_plan: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub sell_sum_fact: Option<Decimal>,

	#[schema(value_type = Decimal)]
	#[serde(with = "rust_decimal::serde::float_option")]
	pub sell_nds_sum_fact: Option<Decimal>,

	pub implementation_id: Option<Uuid>,
	pub user_id: Uuid,
	pub date_created: DateTime<Utc>,
	pub date_updated: Option<DateTime<Utc>>,
	pub deleted: bool,
}
