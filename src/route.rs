use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_wordPair_handler,
        delete_wordPair_handler,
        edit_wordPair_handler,
        get_wordPair_handler,
        route_options_handler,
        wordPairs_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::word_pair_db();

    Router::new()
        .route("/", get(route_options_handler))
        .route(
            "/word-pairs",
            post(create_wordPair_handler).get(wordPairs_list_handler),
        )
        .route(
            "/word-pairs/:id",
            get(get_wordPair_handler)
                .patch(edit_wordPair_handler)
                .delete(delete_wordPair_handler),
        )
        .with_state(db)
}
