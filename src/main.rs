use axum::{Json, response::IntoResponse, Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(display_language_options))
        .route("/language/spanish", get(spanish));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn display_language_options() -> impl IntoResponse {
    const MESSAGE: &str =
"Beginner words in foreign languages.
URL options:
    /spanish
    /german
    /romanian";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

async fn spanish() -> impl IntoResponse {
    const MESSAGE: &str =
"Hi/Hello: Hola
Bye/Goodbye: Adios
Yes: Sí
No: No
Please: Por favor
Thank you: Gracias
You're welcome: De nada";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
