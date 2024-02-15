use axum::{
    extract::{Path, State},
    Json,
};

use tracing::info;

use crate::{
    server::ServerState,
    Error,
};

use namada_sdk::types::token::Transfer;

use sqlx::Row as TRow;

pub async fn get_transfers_by_source(
    State(state): State<ServerState>,
    Path(source): Path<String>,
) -> Result<Json<Vec<Transfer>>, Error> {
    info!("calling /transfers/from");
    Ok(Json(vec![]))
}

pub async fn get_transfers_by_target(
    State(state): State<ServerState>,
    Path(target): Path<String>,
) -> Result<Json<Vec<Transfer>>, Error> {
    info!("calling /transfers/to");
    Ok(Json(vec![]))
}

pub async fn get_transfers_by_source_or_target(
    State(state): State<ServerState>,
    Path(participant): Path<String>,
) -> Result<Json<Vec<Transfer>>, Error> {
    info!("calling /transfers/by");
    Ok(Json(vec![]))
}

pub async fn get_transfers_by_block_hash(
    State(state): State<ServerState>,
    Path(hash): Path<String>,
) -> Result<Json<Vec<Transfer>>, Error> {
    info!("calling /transfers/in");
    Ok(Json(vec![]))
}

pub async fn get_transfers_by_block_height(
    State(state): State<ServerState>,
    Path(height): Path<u32>,
) -> Result<Json<Vec<Transfer>>, Error> {
    info!("calling /transfers/at");
    Ok(Json(vec![]))
}
