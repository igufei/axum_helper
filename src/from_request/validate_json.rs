use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;
/// 自定义一个提取器,在内部实现数据验证
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let result = Json::<T>::from_request(req, state).await;
        match result {
            Ok(Json(value)) => {
                if let Err(err) = value.validate() {
                    return Err((StatusCode::BAD_REQUEST, err.to_string()));
                }
                Ok(ValidatedJson(value))
            }
            Err(e) => {
                println!("{}", e);
                Err((StatusCode::BAD_REQUEST, e.to_string()))
            }
        }
    }
}
