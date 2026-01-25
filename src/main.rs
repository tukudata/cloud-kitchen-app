mod models;
mod routes;
mod handlers;

use axum::Router;
use std::sync::{Arc, Mutex};
use crate::routes::menu::menu_routes;

#[tokio::main]
async fn main() {
    // Inisialisasi Database palsu (In-memory)
    let db = Arc::new(Mutex::new(Vec::new()));

    // Gabungkan rute menu ke dalam aplikasi utama
    let app = Router::new()
        .nest("/api", menu_routes(db)); // Semua url diawali /api

    // Alamat server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🔥 Cloud Kitchen API meluncur di http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}