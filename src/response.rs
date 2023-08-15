use crate::models::word_pair_model::WordPair;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct WordPairData {
    pub word_pair: WordPair,
}
