use std::sync::Arc;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::jwt::Claims;

/// 获取当前登录的用户凭证
/// 这个功能需要使用auth中间件后,被正确的处理后才能获取
pub struct User(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let result = parts.extensions.get::<Arc<Claims>>();
        match result {
            Some(value) => Ok(User(value.sub.clone())),
            None => Err((
                StatusCode::BAD_REQUEST,
                "用户凭证无法获取,请重启登录".to_string(),
            )),
        }
    }
}
