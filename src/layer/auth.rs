use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::jwt::JWT;

//use crate::{configs::JWT_SECRET};
// 这里实现一request 接收器 ,这里可以使用from_extractor::<Authlayer>()转成中间件

// 这里使用fn来实现一个中间件,使用from_fn转
pub async fn auth(secret: &str, mut request: Request, next: Next) -> Response {
    let headers = request.headers();
    let auth_header = headers.get("Authorization");
    let token = match auth_header {
        Some(v) => {
            let v = v.to_str().unwrap();
            let parts: Vec<&str> = v.split(' ').collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                parts[1].to_string()
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    };
    let result = JWT::decode(&token, secret);
    match result {
        Ok(value) => {
            let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let timestamp =
                duration.as_secs() * 1000 + u64::from(duration.subsec_nanos()) / 1_000_000;
            if value.exp < timestamp {
                return (StatusCode::UNAUTHORIZED, "登录过期,请重新登录!").into_response();
            }
            request.extensions_mut().insert(Arc::new(value));
            next.run(request).await
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "登录过期,请重新登录!").into_response(),
    }
}
