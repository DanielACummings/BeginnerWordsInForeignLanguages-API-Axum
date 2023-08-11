use axum::{response::IntoResponse, routing::get, Json, Router};

async fn display_language_options() -> impl IntoResponse {
    const MESSAGE: &str =
"Beginner words in foreign languages.
Language options:
    1. Spanish
    2. German
    3. Romanian";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/home", get(display_language_options));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
