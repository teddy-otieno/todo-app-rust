
use axum::extract::Path;
use axum::extract::State;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use models::MarkDoneChange;
use models::NewTodo;
use models::Todo;
use std::net::SocketAddr;
use axum::{Router, routing::get, http::StatusCode, extract::Json, response::IntoResponse};
use std::env; 

use tower_http::cors::{Any, CorsLayer};

mod models;
mod schema;

#[axum_macros::debug_handler]
async fn get_todos(State(state): State<AppState>) -> impl IntoResponse {
    let mut conn = state.conn_pool.get().unwrap();
    let todos = crate::schema::todo::dsl::todo
        .load::<Todo>(&mut conn)
        .unwrap();

    (StatusCode::OK, Json(todos))
}

#[axum_macros::debug_handler]
async fn add_todo(State(state): State<AppState>, Json(payload): Json<NewTodo>) -> impl IntoResponse {
    let mut conn = state.conn_pool.get().unwrap();

    let new_record: Todo = diesel::insert_into(schema::todo::table)
        .values(&payload)
        .get_result(&mut conn)
        .unwrap();

    (StatusCode::CREATED, Json(new_record))
}


async fn mark_done(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let mut conn = state.conn_pool.get().unwrap();
    let new_record = MarkDoneChange { id : id, done: true };
    let new = new_record.save_changes::<Todo>(&mut conn).unwrap();
    (StatusCode::OK, Json(new))
}

fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be supplied");

    let connection_manager = ConnectionManager::new(db_url);
    Pool::builder()
        .max_size(10)
        .build(connection_manager)
        .unwrap()

}

#[derive(Clone, )]
struct AppState {
    conn_pool: Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let shared_state = AppState {
        conn_pool: establish_connection()
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(get_todos).post(add_todo))
        .route("/done/:id", get(mark_done))
        .with_state(shared_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to create application");
}
