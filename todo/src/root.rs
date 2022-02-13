use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html("Hello, World!")
}
