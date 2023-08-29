use crate::model::WordPair;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct WordPairData {
    pub wordPair: WordPair,
}

#[derive(Serialize, Debug)]
pub struct SingleWordPairResponse {
    pub status: String,
    pub data: WordPairData,
}

#[derive(Serialize, Debug)]
pub struct WordPairListResponse {
    pub status: String,
    pub results: usize,
    pub wordPairs: Vec<WordPair>,
}
