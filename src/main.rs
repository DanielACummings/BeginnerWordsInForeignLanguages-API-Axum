mod handler;
mod model;
mod response;
mod route;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Made using code from
// https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
// as a template

// Run server & auto-update when code changes are made:
// cargo watch -q -c -w src/ -x run

// async fn display_language_options() -> impl IntoResponse {
//     const MESSAGE: &str =
// "Beginner words in foreign languages.
// URL options:
//     /spanish
//     /german
//     /romanian";

//     let json_response = serde_json::json!({
//         "status": "success",
//         "message": MESSAGE
//     });

//     Json(json_response)
// }

// async fn spanish() -> impl IntoResponse {
//     const MESSAGE: &str =
// "Hi/Hello: Hola
// Bye/Goodbye: Adios
// Yes: Sí
// No: No
// Please: Por favor
// Thank you: Gracias
// You're welcome: De nada";

//     let json_response = serde_json::json!({
//         "status": "success",
//         "message": MESSAGE
//     });

//     Json(json_response)
// }
