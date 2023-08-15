use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{
        query_options_model::QueryOptions,
        update_word_pair_schema::UpdateWordPairSchema},
        word_pair_model::DB,
        word_pair_model::WordPair,
    response::{SingleWordPairResponse, WordPairData, WordPairListResponse},
};

pub async fn word_pairs_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let word_pairs = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let word_pairs: Vec<WordPair> = word_pairs.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = WordPairListResponse {
        results: word_pairs.len(),
        word_pairs,
    };

    Json(json_response)
}

pub async fn create_word_pair_handler(
    State(db): State<DB>,
    Json(mut body): Json<WordPair>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(word_pair) = vec.iter().find(|word_pair| word_pair.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Word pair with English word, \"{}\", already exists", word_pair.english_word),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let word_pair = body.to_owned();

    vec.push(body);

    let json_response = SingleWordPairResponse {
        data: WordPairData { word_pair },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_word_pair_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(word_pair) = vec.iter().find(|word_pair| word_pair.id == Some(id.to_owned())) {
        let json_response = SingleWordPairResponse {
            status: "success".to_string(),
            data: WordPairData { word_pair: word_pair.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Word pair with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
