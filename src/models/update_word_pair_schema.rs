use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}
