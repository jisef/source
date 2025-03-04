use askama::Template;
use axum::{response::Html, routing::get, serve, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

/// Application State
#[derive(Clone)]
struct AppState {
    name: String,
}

/// Askama-Template für die Startseite
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        name: "Axum Alpine Tailwind".to_string(),
    });

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static")) // Statische Dateien
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "✅ Server läuft auf http://{}",
        listener.local_addr().unwrap()
    );

    serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let template = IndexTemplate {
        name: "Axum".into(),
    };
    Html(template.render().unwrap())
}
