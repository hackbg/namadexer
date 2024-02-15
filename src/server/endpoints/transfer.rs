use axum::{extract::{Path, State}, Json};
use crate::{server::ServerState, Error};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;
use sqlx::postgres::PgRow as Row;
use tracing::{info, instrument};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferRecord {
    tx_id:                   String,
    source:                  String,
    target:                  String,
    token:                   String,
    amount:                  String,
    key:                     String,
    shielded:                String,
    block_id:                String,
    tx_type:                 String,
    wrapper_id:              String,
    fee_amount_per_gas_unit: String,
    fee_token:               String,
    gas_limit_multiplier:    String,
    header_height:           String,
    header_time:             String,
}

impl TryFrom<&Row> for TransferRecord {
    type Error = Error;

    #[instrument(level = "trace", skip(row))]
    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            tx_id:                   row.try_get("tx_id")?,
            source:                  row.try_get("source")?,
            target:                  row.try_get("target")?,
            token:                   row.try_get("token")?,
            amount:                  row.try_get("amount")?,
            key:                     row.try_get("key")?,
            shielded:                row.try_get("shielded")?,
            block_id:                row.try_get("block_id")?,
            tx_type:                 row.try_get("tx_type")?,
            wrapper_id:              row.try_get("wrapper_id")?,
            fee_amount_per_gas_unit: row.try_get("fee_amount_per_gas_unit")?,
            fee_token:               row.try_get("fee_token")?,
            gas_limit_multiplier:    row.try_get("gas_limit_multiplier")?,
            header_height:           row.try_get("header_height")?,
            header_time:             row.try_get("header_time")?,
        })
    }
}

pub async fn get_transfers_by_source(
    State(state): State<ServerState>,
    Path(source): Path<String>,
) -> Result<Json<Vec<TransferRecord>>, Error> {
    info!("calling /transfers/from");
    let mut rows: Vec<TransferRecord> = vec![];
    for row in state.db.list_transfers_by_source(source).await?.iter() {
        rows.push(row.try_into()?);
    }
    Ok(Json(rows))
}

pub async fn get_transfers_by_target(
    State(state): State<ServerState>,
    Path(target): Path<String>,
) -> Result<Json<Vec<TransferRecord>>, Error> {
    info!("calling /transfers/to");
    let mut rows: Vec<TransferRecord> = vec![];
    for row in state.db.list_transfers_by_target(target).await?.iter() {
        rows.push(row.try_into()?);
    }
    Ok(Json(rows))
}

pub async fn get_transfers_by_source_or_target(
    State(state): State<ServerState>,
    Path(address): Path<String>,
) -> Result<Json<Vec<TransferRecord>>, Error> {
    info!("calling /transfers/by");
    let mut rows: Vec<TransferRecord> = vec![];
    for row in state.db.list_transfers_by_source_or_target(address).await?.iter() {
        rows.push(row.try_into()?);
    }
    Ok(Json(rows))
}

pub async fn get_transfers_by_block_hash(
    State(state): State<ServerState>,
    Path(hash): Path<String>,
) -> Result<Json<Vec<TransferRecord>>, Error> {
    info!("calling /transfers/in");
    let mut rows: Vec<TransferRecord> = vec![];
    for row in state.db.list_transfers_by_block_hash(hash).await?.iter() {
        rows.push(row.try_into()?);
    }
    Ok(Json(rows))
}

pub async fn get_transfers_by_block_height(
    State(state): State<ServerState>,
    Path(height): Path<u32>,
) -> Result<Json<Vec<TransferRecord>>, Error> {
    info!("calling /transfers/at");
    let mut rows: Vec<TransferRecord> = vec![];
    for row in state.db.list_transfers_by_block_height(height).await?.iter() {
        rows.push(row.try_into()?);
    }
    Ok(Json(rows))
}
