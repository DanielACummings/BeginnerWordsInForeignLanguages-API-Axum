use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_todo_handler,
        delete_todo_handler,
        edit_todo_handler,
        get_todo_handler,
        route_options_handler,
        todos_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route("/", get(route_options_handler))
        .route(
            "/todos",
            post(create_todo_handler).get(todos_list_handler),
        )
        .route(
            "/todos/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(db)
}
