use askama::Template;
use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    HomeTemplate
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate;
