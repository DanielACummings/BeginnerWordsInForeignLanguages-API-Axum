use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WordPair {
    pub id: Option<String>,
    pub foreign_language: String,
    pub english_word: String,
    pub foreign_word: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub type DB = Arc<Mutex<Vec<WordPair>>>;

pub fn create_word_pair_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}
