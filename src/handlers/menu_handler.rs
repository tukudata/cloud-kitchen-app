use axum::response::Html;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

// Kita panggil tipenya lengkap dengan alamatnya
pub type Db = Arc<Mutex<Vec<crate::models::item::MenuItem>>>;

pub async fn list_menu(
    State(db): State<Db>,
) -> Json<Vec<crate::models::item::MenuItem>> {
    let menu = db.lock().expect("Gagal mengunci");
    Json(menu.clone())
}

pub async fn create_menu(
    State(db): State<Db>,
    Json(payload): Json<crate::models::item::MenuItem>,
) -> Json<crate::models::item::MenuItem> {
    println!("📥 Ada pesanan baru masuk: {}", payload.nama); // Tambahkan ini
    let mut menu = db.lock().expect("Gagal mengunci");
    menu.push(payload.clone());
    Json(payload)
}

pub async fn render_dashboard(State(db): State<Db>) -> Html<String> {
    let menu = db.lock().unwrap();
    
    // Kita buat string HTML secara dinamis
    let mut rows = String::new();
    for item in menu.iter() {
        rows.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td>Rp {}</td>
                <td>{}</td>
            </tr>",
            item.id, item.nama, item.harga, item.kategori
        ));
    }

    let html_content = format!(
        "<html>
            <head>
                <title>Cloud Kitchen Dashboard</title>
                <style>
                    body {{ font-family: sans-serif; padding: 40px; background: #f4f4f4; }}
                    table {{ width: 100%; border-collapse: collapse; background: white; }}
                    th, td {{ padding: 12px; border: 1px solid #ddd; text-align: left; }}
                    th {{ background: #ff5722; color: white; }}
                    h1 {{ color: #333; }}
                </style>
            </head>
            <body>
                <h1>🍳 Cloud Kitchen Menu Dashboard</h1>
                <table>
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>Nama Menu</th>
                            <th>Harga</th>
                            <th>Kategori</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
                <p><i>Refresh halaman untuk melihat update data terbaru.</i></p>
            </body>
        </html>",
        rows
    );

    Html(html_content)
}