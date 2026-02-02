# 🍳 Rust Cloud Kitchen AI

A simple practice project to explore the **Rust** ecosystem. It uses **Axum** for the backend and **Ollama** for Local AI integration.

## 📸 Preview
![Dashboard](dashboard.png)

## 🚀 Features
* **AI Menu Parser:** Automatically converts raw text into menu data using `Llama 3.2`.
* **Web Dashboard:** A real-time menu list view built with Tailwind CSS.
* **Modern Rust:** Uses `Arc<Mutex>` for safe state management and a modular folder structure.

## 🛠️ Tech Stack
* **Backend:** Rust (Axum)
* **AI:** Ollama (Llama 3.2:1b)
* **Frontend:** HTML + Tailwind CSS

## 📦 How to Run (Step by Step)
1. Start the AI Engine
   Open your **first terminal** and run Ollama. This must stay open:
   ```bash
   ollama run llama3.2:1b
2. Clone this repo:
   ```bash
   git clone https://github.com/tukudata/cloud-kitchen-app.git
3. Start the Rust Server
   Open a **second terminal** tab, go to the project folder, and run:
   ```bash
   cargo run
4. Open the Dashboard
   Open your browser and go to:
   ```plaintext
   http://127.0.0.1:3000/api/dashboard

## 🤖 How to Add Menu via AI
Open a **third terminal** tab and send your menu text using `curl`:

Example:
```bash
curl -X POST http://127.0.0.1:3000/api/menu/ai -d "Nasi Padang 20000 kategori makanan"
curl -X POST http://127.0.0.1:3000/api/menu/ai -d "Jus Alpukat 8000 kategori minuman"
```

The AI will parse your sentence and automatically add it to the dashboard!


