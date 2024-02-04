use crate::{
    handler::{
        create_sarcina_handler, delete_sarcina_handler, get_sarcina_handler,
        health_checker_handler, sarcini_list_handler,
    },
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/sarcini/", post(create_sarcina_handler))
        .route("/api/sarcini", get(sarcini_list_handler))
        .route(
            "/api/sarcini/:id",
            get(get_sarcina_handler).delete(delete_sarcina_handler),
        )
        .with_state(app_state)
}
