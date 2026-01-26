use serde_json::{json, Value};
use reqwest::Client;

pub async fn ask_ai_to_parse_menu(input: &str) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    
    // Ini adalah instruksi agar AI tidak ngaco dan hanya kasih JSON
    let prompt = format!(
        "Ubah teks berikut menjadi JSON untuk menu makanan: '{}'. 
        Format harus JSON: {{\"id\": 1, \"nama\": \"...\", \"harga\": 0.0, \"kategori\": \"...\"}}. 
        Hanya berikan JSON, jangan ada penjelasan lain.",
        input
    );

    let response = client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&json!({
            "model": "llama3.2:1b",
            "prompt": prompt,
            "stream": false,
            "format": "json" // Memaksa Ollama kasih format JSON
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Mengambil bagian "response" dari JSON Ollama
    Ok(response["response"].clone())
}