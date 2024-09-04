CREATE TABLE "transactions" (
	"id" uuid DEFAULT gen_random_uuid(),
	"op_date" TIMESTAMPTZ NOT NULL,
	"gas_station_id" uuid NOT NULL,
	"card_id" uuid DEFAULT NULL,
	"contract_id" uuid DEFAULT NULL,
	"nomenclature_id" uuid NOT NULL,
	"amount" numeric(15,2) DEFAULT NULL,
	"stella_sum" numeric(15,2) DEFAULT NULL,
	"stella_nds_sum" numeric(15,2) DEFAULT NULL,
	"refund" boolean NOT NULL DEFAULT false,
	"buy_sum_plan" numeric(15,2) DEFAULT NULL,
	"buy_nds_sum_plan" numeric(15,2) DEFAULT NULL,
	"buy_sum_fact" numeric(15,2) DEFAULT NULL,
	"buy_nds_sum_fact" numeric(15,2) DEFAULT NULL,
	"sell_sum_plan" numeric(15,2) DEFAULT NULL,
	"sell_nds_sum_plan" numeric(15,2) DEFAULT NULL,
	"sell_sum_fact" numeric(15,2) DEFAULT NULL,
	"sell_nds_sum_fact" numeric(15,2) DEFAULT NULL,
	"implementation_id" uuid DEFAULT NULL,
	"user_id" uuid NOT NULL,
	"date_created" TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
	"date_updated" TIMESTAMPTZ DEFAULT (now() at time zone 'utc'),
	"deleted" boolean NOT NULL DEFAULT false,

	CONSTRAINT "PK_transactions" PRIMARY KEY ("id")
);

CREATE TRIGGER "mod_tx_updated"
BEFORE UPDATE ON "transactions"
FOR EACH ROW
EXECUTE PROCEDURE moddatetime ("date_updated");
