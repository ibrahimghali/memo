use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct WordEntry {
    pub id: String,
    pub word: String,
    pub translation_en: String,
    pub translation_ar: String,
    pub example: String,
    pub part_of_speech: String,    
    pub register: String,           
    pub synonyms: String,          
    pub notes: String,              
    pub created_at: Option<String>,
}