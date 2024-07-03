use axum::{extract::Request, middleware::Next, response::Response};
use chrono::{Local, Utc};

pub async fn log(request: Request, next: Next) -> Response {
    // 获取当前时间
    let now = Utc::now();

    // 格式化为本地时间字符串
    let local_time = now.with_timezone(&Local);
    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let method = request.method();
    let uri = request.uri();
    println!("{} -> {} {}", formatted_time, method, uri);

    next.run(request).await
}
