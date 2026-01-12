use axum::{Router, routing::post, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use bitcoin::PublicKey;

use crate::bitcoin::psbt_verify;
use crate::bitcoin::signing_registry;
use crate::domain::signing::SignerRole;

#[derive(Deserialize)]
pub struct SubmitSignedPsbtRequest {
    pub escrow_id: String,
    pub signer_role: String,
    pub psbt_base64: String,

    pub borrower_pubkey: String,
    pub lender_pubkey: String,
    pub escrow_pubkey: String,
}

#[derive(Serialize)]
pub struct SubmitSignedPsbtResponse {
    pub status: String,
    pub signatures_collected: usize,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn parse_role(role: &str) -> Option<SignerRole> {
    match role {
        "BORROWER" => Some(SignerRole::Borrower),
        "LENDER" => Some(SignerRole::Lender),
        "ESCROW" => Some(SignerRole::Escrow),
        _ => None,
    }
}

pub fn routes() -> Router {
    Router::new().route("/psbt/submit-signed", post(submit_signed))
}

async fn submit_signed(
    Json(req): Json<SubmitSignedPsbtRequest>
) -> Result<Json<SubmitSignedPsbtResponse>, (StatusCode, Json<ErrorResponse>)> {

    let role = parse_role(&req.signer_role)
        .ok_or((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Invalid signer role".into() })
        ))?;

    let pubkeys = vec![
        req.borrower_pubkey.parse::<PublicKey>(),
        req.lender_pubkey.parse::<PublicKey>(),
        req.escrow_pubkey.parse::<PublicKey>(),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse { error: "Invalid public key".into() })
    ))?;

    // Structural verification (no crypto sighash yet)
    psbt_verify::verify_2of3_psbt(&req.psbt_base64, &pubkeys)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: format!("{:?}", e) })
        ))?;

    let state = signing_registry::record_signature(&req.escrow_id, role);

    let status = if state.is_approved() {
        "APPROVED"
    } else {
        "PARTIALLY_SIGNED"
    };

    Ok(Json(SubmitSignedPsbtResponse {
        status: status.to_string(),
        signatures_collected: state.signed_roles.len(),
    }))
}