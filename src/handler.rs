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
            "/wordPairs": {
                "GET": {
                    "description": "List all wordPairs",
                    "parameters": "None"
                },
                "POST": {
                    "description": "Create wordPair",
                    "parameters": "title (unique), content"
                }
            },
            "/wordPairs/:id": {
                "GET": {
                    "description": "Get wordPair",
                    "parameterers": "id"
                },
                "PATCH": {
                    "description": "Update wordPair",
                    "parameters": "id, title (optional), content (optional)"
                },
                "DELETE": {
                    "description": "Delete wordPair",
                    "parameters": "None"
                }
            },
        }
    });

    Json(json_response)
}

pub async fn wordPairs_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let wordPairs = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let wordPairs: Vec<WordPair>
        = wordPairs.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = WordPairListResponse {
        status: "success".to_string(),
        results: wordPairs.len(),
        wordPairs,
    };

    Json(json_response)
}

pub async fn create_wordPair_handler(
    State(db): State<DB>,
    Json(mut body): Json<WordPair>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(wordPair) = vec.iter().find(
        |wordPair| wordPair.title == body.title
    ) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!(
                "WordPair with title: '{}' already exists",
                wordPair.title
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

    let wordPair = body.to_owned();

    vec.push(body);

    let json_response = SingleWordPairResponse {
        status: "success".to_string(),
        data: WordPairData { wordPair },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_wordPair_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(wordPair) = vec.iter().find(
        |wordPair| wordPair.id == Some(id.to_owned())
    ) {
        let json_response = SingleWordPairResponse {
            status: "success".to_string(),
            data: WordPairData { wordPair: wordPair.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("WordPair with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn edit_wordPair_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateWordPairSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(wordPair) = vec.iter_mut().find(
        |wordPair| wordPair.id == Some(id.clone())
    ) {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| wordPair.title.to_owned());
        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| wordPair.content.to_owned());
        let favorite = body.favorite.unwrap_or(
            wordPair.favorite.unwrap()
        );
        let payload = WordPair {
            id: wordPair.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                wordPair.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                wordPair.content.to_owned()
            },
            favorite: Some(favorite),
            created_at: wordPair.created_at,
            updated_at: Some(datetime),
        };
        *wordPair = payload;

        let json_response = SingleWordPairResponse {
            status: "success".to_string(),
            data: WordPairData { wordPair: wordPair.clone() },
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

pub async fn delete_wordPair_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(
        |wordPair| wordPair.id == Some(id.clone())
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
