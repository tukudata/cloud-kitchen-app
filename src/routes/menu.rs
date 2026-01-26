use axum::{routing::{get, post}, Router};
use crate::handlers::menu_handler::{list_menu, create_menu, render_dashboard, create_menu_by_ai, Db};

pub fn menu_routes(state: Db) -> Router {
    Router::new()
        .route("/menu", get(list_menu)) 
        .route("/menu", post(create_menu))
        .route("/menu/ai", post(create_menu_by_ai))
        .route("/dashboard", get(render_dashboard))
        .with_state(state)                  // Kasih akses ke "Database"
}