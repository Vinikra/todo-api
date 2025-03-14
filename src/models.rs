use serde::{Serialize, Deserialize};
use chrono::NaiveDataTime;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Task{
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}