use axum::{Router, routing::post, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use base64::Engine;
use bitcoin::PublicKey;

use crate::bitcoin::{psbt, psbt_verify};

//
// ---------- CREATE PSBT ----------
//

#[derive(Deserialize)]
pub struct PsbtRequest {
    pub unsigned_tx_hex: String,
}

#[derive(Serialize)]
pub struct PsbtResponse {
    pub psbt_base64: String,
}

async fn create_psbt(
    Json(req): Json<PsbtRequest>
) -> Result<Json<PsbtResponse>, (StatusCode, Json<ErrorResponse>)> {

    match psbt::create_psbt_from_hex(&req.unsigned_tx_hex) {
        Ok(psbt) => Ok(Json(PsbtResponse {
            psbt_base64: base64::engine::general_purpose::STANDARD
                .encode(psbt.serialize()),
        })),

        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid unsigned transaction: {:?}", e),
            }),
        )),
    }
}

//
// ---------- VERIFY PSBT ----------
//

#[derive(Deserialize)]
pub struct VerifyPsbtRequest {
    pub psbt_base64: String,
    pub borrower_pubkey: String,
    pub lender_pubkey: String,
    pub escrow_pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyPsbtResponse {
    pub valid: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

async fn verify_psbt(
    Json(req): Json<VerifyPsbtRequest>
) -> Result<Json<VerifyPsbtResponse>, (StatusCode, Json<ErrorResponse>)> {

    let pubkeys = vec![
        req.borrower_pubkey.parse::<PublicKey>(),
        req.lender_pubkey.parse::<PublicKey>(),
        req.escrow_pubkey.parse::<PublicKey>(),
    ]
    .into_iter()
    .map(|k| {
        k.map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid public key".to_string()
            })
        ))
    })
    .collect::<Result<Vec<_>, _>>()?;

    match psbt_verify::verify_2of3_psbt(&req.psbt_base64, &pubkeys) {
        Ok(_) => Ok(Json(VerifyPsbtResponse { valid: true })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("{:?}", e),
            }),
        )),
    }
}

//
// ---------- ROUTES ----------
//

pub fn routes() -> Router {
    Router::new()
        .route("/transaction/create-psbt", post(create_psbt))
        .route("/transaction/verify-psbt", post(verify_psbt))
}
