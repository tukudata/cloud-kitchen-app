use serde::{Deserialize, Serialize, Deserializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    pub id: u32,
    pub nama: String,
    #[serde(deserialize_with = "flexible_float")]
    pub harga: f64,
    pub kategori: String,
}

fn flexible_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        // If it's already a number, just return it
        serde_json::Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0)),
        
        // If it's a string (e.g., "8.000" or "Rp 8000")
        serde_json::Value::String(s) => {
            let clean_s: String = s.chars()
                .filter(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            
            clean_s.parse::<f64>().map_err(serde::de::Error::custom)
        },
        _ => Ok(0.0),
    }
}