use axum::{routing::{get, post}, Router, extract::State, Json};
use crate::{db::{create_task, get_tasks}, models::{Task, User, NewUser}, auth};

pub fn app(pool: sqlx::SqlitePool) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(pool)
}

pub async fn create_task_handler(
    State(pool): State<sqlx::SqlitePool>,
    axum::extract::Extension(user_id): axum::extract::Extension<String>,
    Json(mut task): Json<Task>,
) -> Result<Json<Task>, axum::http::StatusCode> {
    task.user_id = user_id.parse().unwrap();
    let new_task = create_task(&pool, task).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(new_task))
}

pub async fn list_tasks_handler(
    State(pool): State<sqlx::SqlitePool>,
    axum::extract::Extension(user_id): axum::extract::Extension<String>,
) -> Result<Json<Vec<Task>>, axum::http::StatusCode> {
    let tasks = get_tasks(&pool, user_id.parse().unwrap())
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tasks))
}

pub async fn register_handler(
    State(pool): State<sqlx::SqlitePool>,
    Json(user): Json<crate::models::NewUser>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let password_hash = auth::hash_password(&user.password_hash);
    sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&password_hash)
        .execute(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    Ok(Json("User registered".to_string()))
}

pub async fn login_handler(
    State(pool): State<sqlx::SqlitePool>,
    Json(user): Json<crate::models::User>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let db_user = sqlx::query_as::<_, crate::models::User>("SELECT * FROM users WHERE username = ?")
        .bind(&user.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    
    if auth::verify_password(&db_user.password_hash, &user.password_hash) {
        let token = auth::create_token(&db_user.id.to_string());
        Ok(Json(token))
    } else {
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}