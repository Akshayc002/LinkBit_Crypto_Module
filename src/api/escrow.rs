use axum::{Router, routing::post, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use bitcoin::PublicKey;

use crate::bitcoin::multisig;
use crate::config;

#[derive(Deserialize)]
pub struct CreateEscrowRequest {
    pub borrower_pubkey: String,
    pub lender_pubkey: String,
    pub escrow_pubkey: String,
}

#[derive(Serialize)]
pub struct CreateEscrowResponse {
    pub escrow_address: String,
    pub redeem_script: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn routes() -> Router {
    Router::new().route("/escrow/create", post(create_escrow))
}

async fn create_escrow(
    Json(req): Json<CreateEscrowRequest>
) -> Result<Json<CreateEscrowResponse>, (StatusCode, Json<ErrorResponse>)> {

    let borrower = req.borrower_pubkey.parse::<PublicKey>()
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Invalid borrower public key".into() })
        ))?;

    let lender = req.lender_pubkey.parse::<PublicKey>()
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Invalid lender public key".into() })
        ))?;

    let escrow = req.escrow_pubkey.parse::<PublicKey>()
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Invalid escrow public key".into() })
        ))?;

    let (address, script) =
        multisig::create_2of3_multisig(
            vec![borrower, lender, escrow],
            config::bitcoin_network()
        );

    Ok(Json(CreateEscrowResponse {
        escrow_address: address.to_string(),
        redeem_script: hex::encode(script.as_bytes()),
    }))
}