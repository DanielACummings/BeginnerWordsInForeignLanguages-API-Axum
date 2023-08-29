use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    model::{QueryOptions, WordPair, UpdateWordPairSchema, DB},
    response::{SingleWordPairResponse, WordPairData, WordPairListResponse},
};

pub async fn route_options_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
        "available_routes": {
            "/word-pairs": {
                "GET": {
                    "description": "List all word pairs",
                    "parameters": "None"
                },
                "POST": {
                    "description": "Create word pair",
                    "parameters": "title (unique), content"
                }
            },
            "/word-pairs/:id": {
                "GET": {
                    "description": "Get word pair",
                    "parameterers": "id"
                },
                "PATCH": {
                    "description": "Update word pair",
                    "parameters": "id, title (optional), content (optional)"
                },
                "DELETE": {
                    "description": "Delete word pair",
                    "parameters": "None"
                }
            },
        }
    });

    Json(json_response)
}

pub async fn word_pairs_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let word_pairs = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let word_pairs: Vec<WordPair>
        = word_pairs.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = WordPairListResponse {
        status: "success".to_string(),
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

    if let Some(word_pair) = vec.iter().find(
        |word_pair| word_pair.title == body.title
    ) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!(
                "WordPair with title: '{}' already exists",
                word_pair.title
            ),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.favorite = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let word_pair = body.to_owned();

    vec.push(body);

    let json_response = SingleWordPairResponse {
        status: "success".to_string(),
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

    if let Some(word_pair) = vec.iter().find(
        |word_pair| word_pair.id == Some(id.to_owned())
    ) {
        let json_response = SingleWordPairResponse {
            status: "success".to_string(),
            data: WordPairData { word_pair: word_pair.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("WordPair with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn edit_word_pair_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateWordPairSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(word_pair) = vec.iter_mut().find(
        |word_pair| word_pair.id == Some(id.clone())
    ) {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| word_pair.title.to_owned());
        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| word_pair.content.to_owned());
        let favorite = body.favorite.unwrap_or(
            word_pair.favorite.unwrap()
        );
        let payload = WordPair {
            id: word_pair.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                word_pair.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                word_pair.content.to_owned()
            },
            favorite: Some(favorite),
            created_at: word_pair.created_at,
            updated_at: Some(datetime),
        };
        *word_pair = payload;

        let json_response = SingleWordPairResponse {
            status: "success".to_string(),
            data: WordPairData { word_pair: word_pair.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("WordPair with ID: {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn delete_word_pair_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(
        |word_pair| word_pair.id == Some(id.clone())
    ) {
        vec.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("WordPair with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
