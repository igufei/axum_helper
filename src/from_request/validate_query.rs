use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};
use serde::de::DeserializeOwned;
use validator::Validate;
/// 自定义一个提取器,在内部实现数据验证
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let value = serde_urlencoded::from_str::<T>(query);
        match value {
            Ok(v) => {
                if let Err(err) = v.validate() {
                    return Err((StatusCode::BAD_REQUEST, err.to_string()));
                }
                Ok(ValidatedQuery(v))
            }
            Err(e) => {
                println!("{}", e);
                Err((StatusCode::BAD_REQUEST, e.to_string()))
            }
        }
    }
}