mod models;
mod routes;
mod db;
mod auth;
mod middleware;

#[tokio::main]
async fn main() {
    let pool = db::init_pool().await;

    // Rotas públicas (sem autenticação)
    let public_routes = routes::app(pool.clone());

    // Rotas protegidas (com autenticação)
    let protected_routes = axum::Router::new()
        .route("/tasks", axum::routing::post(routes::create_task_handler))
        .route("/tasks", axum::routing::get(routes::list_tasks_handler))
        .with_state(pool.clone())
        .layer(axum::middleware::from_fn(middleware::auth_middleware));

    // Combina todas as rotas
    let app = axum::Router::new()
        .merge(public_routes)
        .merge(protected_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}