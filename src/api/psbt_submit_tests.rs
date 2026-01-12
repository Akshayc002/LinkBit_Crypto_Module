use axum::http::{Request, StatusCode};
use tower::Service;
use crate::api::routes;

#[tokio::test]
async fn submit_signed_psbt_rejects_invalid_role() {
    let app = routes();

    let req = Request::builder()
        .method("POST")
        .uri("/psbt/submit-signed")
        .header("content-type", "application/json")
        .body(r#"{
            "escrow_id": "escrow-1",
            "signer_role": "HACKER",
            "psbt_base64": "cHNidP8BA...",
            "borrower_pubkey": "02...",
            "lender_pubkey": "03...",
            "escrow_pubkey": "02..."
        }"#.into())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}