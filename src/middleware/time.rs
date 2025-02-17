use axum::{extract::Request, middleware::Next, response::Response};
use tokio::time::Instant;

pub async fn time(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let mut res = next.run(request).await;

    let elapsed = format!("{}us", start.elapsed().as_micros());

    res.headers_mut()
        .insert("server-time", elapsed.parse().unwrap());

    res
}
