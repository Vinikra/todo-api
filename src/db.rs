use sqlx::{SqlitePool, migrate::MigrateDatabase};
use crate::models::Task;

pub async fn init_pool() -> SqlitePool {
    let db_url = "sqlite:// tasks.db";
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap() {
        sqlx::Sqlite::create_database(db_url).await.unwrap();
    }
    let pool = SqlitePool::connect(db_url).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        ); 
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            due_date DATETIME,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

pub async fn create_task(pool: &SqlitePool, task: Task) -> Result<Task, sqlx::Error> {
    let result = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (user_id, title, description, status, due_date) 
         VALUES (?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(task.user_id)
    .bind(task.title)
    .bind(task.description)
    .bind(task.status)
    .bind(task.due_date)
    .fetch_one(pool)
    .await?;
    Ok(result)
}

pub async fn get_tasks(pool: &SqlitePool, user_id: i32) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(tasks)
}