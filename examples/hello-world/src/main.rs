//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-hello-world
//! ```

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let router = router();

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test_helpers::*;

    #[tokio::test]
    async fn test_main_router() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/").await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "<h1>Hello, World!</h1>");
    }
}
