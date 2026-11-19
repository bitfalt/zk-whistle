//! Shared HTTP surface for both modules. Each service passes its own scope.
//! No handler reads or logs client IPs or forwarding headers.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::Serialize;
use zk::{Nullifier, ProofRequest, Scope};

#[derive(Clone)]
struct AppState {
    scope: Scope,
    // TODO: move to this module's own Postgres.
    nullifiers: Arc<Mutex<HashSet<Nullifier>>>,
}

#[derive(Serialize)]
struct Verified {
    nullifier: Nullifier,
    new: bool,
}

pub fn router(scope: Scope) -> Router {
    let state = AppState {
        scope,
        nullifiers: Arc::default(),
    };
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/auth/verify", post(verify))
        .with_state(state)
}

async fn verify(
    State(state): State<AppState>,
    Json(proof): Json<ProofRequest>,
) -> Result<Json<Verified>, (StatusCode, String)> {
    let nullifier =
        zk::verify(state.scope, &proof).map_err(|e| (StatusCode::FORBIDDEN, format!("{e:?}")))?;
    let new = state.nullifiers.lock().unwrap().insert(nullifier.clone());
    Ok(Json(Verified { nullifier, new }))
}

pub async fn serve(scope: Scope, default_port: u16) {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(default_port);
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .unwrap();
    println!("{} listening on :{port}", scope.as_str());
    axum::serve(listener, router(scope)).await.unwrap();
}
