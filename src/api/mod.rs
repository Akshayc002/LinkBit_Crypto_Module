use axum::Router;
pub mod escrow;
pub mod transaction;
pub mod health;

// DEV ONLY
pub mod dev;

pub fn routes() -> Router {
    Router::new()
        .merge(escrow::routes())
        .merge(transaction::routes())
        .merge(health::routes())
        .merge(dev::routes())
}
