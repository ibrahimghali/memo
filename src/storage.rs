use reqwest::Client;
use std::env;

use crate::models::WordEntry;

pub async fn add_word(entry: &WordEntry) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/rest/v1/words", env::var("SUPABASE_URL")?);
    let api_key = env::var("SUPABASE_ANON_KEY")?;

    let client = Client::new();
    let res = client
        .post(&url)
        .header("apikey", &api_key)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation") // pour récupérer l’élément créé
        .json(&entry)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let text = res.text().await?;
        Err(format!("Supabase error: {}", text).into())
    }
}

pub async fn load_words() -> Result<Vec<WordEntry>, Box<dyn std::error::Error>> {
    let url = format!("{}/rest/v1/words?select=*", env::var("SUPABASE_URL")?);
    let api_key = env::var("SUPABASE_ANON_KEY")?;

    let client = Client::new();
    let res = client
        .get(&url)
        .header("apikey", &api_key)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    if res.status().is_success() {
        let words = res.json::<Vec<WordEntry>>().await?;
        Ok(words)
    } else {
        let text = res.text().await?;
        Err(format!("Supabase error: {}", text).into())
    }
}