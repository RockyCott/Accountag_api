use axum::{extract::Path, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub async fn path_variables(Path((id, name)): Path<(i32, String)>) -> Json<User> {
    let user = User { id, name };
    Json(user)
}