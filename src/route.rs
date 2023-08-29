use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_word_pair_handler,
        delete_word_pair_handler,
        edit_word_pair_handler,
        get_word_pair_handler,
        route_options_handler,
        word_pairs_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::word_pair_db();

    Router::new()
        .route("/", get(route_options_handler))
        .route(
            "/word-pairs",
            post(create_word_pair_handler).get(word_pairs_list_handler),
        )
        .route(
            "/word-pairs/:id",
            get(get_word_pair_handler)
                .patch(edit_word_pair_handler)
                .delete(delete_word_pair_handler),
        )
        .with_state(db)
}
