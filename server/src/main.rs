use axum::{Router, http::StatusCode, response::Html, routing::get};
use tokio::fs;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/hello-world", get(hello_world))
        .route("/favicon.ico", get(favicon))
        .nest_service("/static", ServeDir::new("server/static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    println!("Running on http://0.0.0.0:8081");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Result<Html<String>, StatusCode> {
    let index = fs::read_to_string("server/templates/index.html")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(index))
}

async fn hello_world() -> Html<String> {
    std::thread::sleep(std::time::Duration::from_secs(1)); // 😪
    Html("🤗Hello World!".to_string())
}

async fn favicon() -> axum::response::Redirect {
    axum::response::Redirect::permanent("/static/svg/icon.svg")
}
