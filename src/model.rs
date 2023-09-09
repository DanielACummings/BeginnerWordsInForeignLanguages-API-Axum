use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WordPair {
    pub id: Option<String>,
    pub english_word: String,
    pub foreign_word: String,
    pub favorite: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub type DB = Arc<Mutex<Vec<WordPair>>>;

pub fn word_pair_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateWordPairSchema {
    pub english_word: Option<String>,
    pub foreign_word: Option<String>,
    pub favorite: Option<bool>,
}
