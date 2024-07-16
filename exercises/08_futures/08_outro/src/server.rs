use std::sync::{Arc, RwLock};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::json;

use crate::dto::{TicketDraft, TicketPatch};
use crate::models::{Ticket, TicketId};
use crate::store::{TicketStore, TicketStoreError};

/// アプリステート
type SharedState = Arc<RwLock<TicketStore>>;

pub async fn run() {
    let shared_state = SharedState::default();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/tickets", post(register_ticket))
        .route("/tickets/:ticket_id", get(retrieve_ticket))
        .route("/tickets/:ticket_id", patch(update_ticket))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// チケットをチケットストアに登録する。
async fn register_ticket(
    State(state): State<SharedState>,
    Json(payload): Json<TicketDraft>,
) -> impl IntoResponse {
    let id = state.write().unwrap().add_ticket(payload);

    Json(json!({"id": id})).into_response()
}

/// チケットストアからチケットを取得する。
async fn retrieve_ticket(
    State(state): State<SharedState>,
    Path((ticket_id,)): Path<(u64,)>,
) -> impl IntoResponse {
    let ticket_id = TicketId(ticket_id);
    match state.read().unwrap().get(ticket_id) {
        Ok(ticket) => ticket.into_response(),
        Err(e) => e.into_response(),
    }
}

/// チケットに登録されているチケットを更新する。
async fn update_ticket(
    State(state): State<SharedState>,
    Path((ticket_id,)): Path<(u64,)>,
    Json(payload): Json<TicketPatch>,
) -> impl IntoResponse {
    let id = TicketId(ticket_id);
    match state.write().unwrap().update_ticket(id, payload) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => e.into_response(),
    }
}

impl IntoResponse for &Ticket {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for TicketStoreError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Ticket does not exist"),
            Self::VersionNotMatch => (
                StatusCode::BAD_REQUEST,
                "The version of updating ticket is not match",
            ),
        };
        let body = Json(json!({"error": message}));

        (status_code, body).into_response()
    }
}
