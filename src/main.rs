mod models;
mod routes;
mod db;
mod auth;
mod middleware;


#[tokio::main]
async fn main() {
    let pool = db::init_pool().await;
    let app = routes::app(pool).layer(axum::middleware::from_fn(middleware::auth_middleware));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}