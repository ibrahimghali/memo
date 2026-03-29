use reqwest::Client;
use serde_json::Value;

use crate::models::WordEntry;

pub async fn generate_entry(word: &str, api_key: &str) -> Result<WordEntry, Box<dyn std::error::Error>> {
    let client = Client::new();

    let prompt = format!(
        "You are an English teacher.
        Word: {}
        Language: French
        
        Return JSON format exactly like this:
        {{
          \"translation_en\": \"english translation here\",
          \"translation_ar\": \"arabic translation here\",
          \"example\": \"example sentence here\",
          \"part_of_speech\": \"noun / verb / adj / etc\",
          \"register\": \"formal / informal / neutral\",
          \"synonyms\": [\"synonym1\", \"synonym2\"],
          \"notes\": \"usage tips here\"
        }}",
        word
    );

    let url = format!(
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
    api_key
);

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let response: Value = client
        .post(url)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    if let Some(err) = response.get("error") {
        return Err(format!("API Error: {}", err).into());
    }

    let mut text = response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| format!("Unexpected API response format: {:?}", response))?;

    text = text.trim();
    if text.starts_with("```json") {
        text = &text[7..];
    } else if text.starts_with("```") {
        text = &text[3..];
    }
    if text.ends_with("```") {
        text = &text[..text.len() - 3];
    }
    text = text.trim();

    let parsed: Value = serde_json::from_str(text)?;

    let synonyms_vec: Vec<String> = parsed["synonyms"].as_array().unwrap_or(&vec![]).iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();

    let entry = WordEntry {
        id: chrono::Utc::now().to_rfc3339(),
        word: word.to_string(),
        translation_en: parsed["translation_en"].as_str().unwrap_or("").to_string(),
        translation_ar: parsed["translation_ar"].as_str().unwrap_or("").to_string(),
        example: parsed["example"].as_str().unwrap_or("").to_string(),
        part_of_speech: parsed["part_of_speech"].as_str().unwrap_or("").to_string(),
        register: parsed["register"].as_str().unwrap_or("").to_string(),
        synonyms: synonyms_vec.join(", "),
        notes: parsed["notes"].as_str().unwrap_or("").to_string(),
        created_at: None,
    };

    Ok(entry)
}