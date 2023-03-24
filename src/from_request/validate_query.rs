use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Query},
    http::{Request, StatusCode},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;
/// 自定义一个提取器,在内部实现数据验证
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection =  (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let result = Query::<T>::from_request(req, state).await;
        match result {
            Ok(Query(value)) => {
                if let Err(err) = value.validate() {
                    return Err((StatusCode::BAD_REQUEST, err.to_string()));
                }
                Ok(ValidatedQuery(value))
            }
            Err(e) => {
                println!("{}",e);
                Err((StatusCode::BAD_REQUEST, "错误:查询字符串缺少参数或格式不正确".to_string()))
            },
        }
    }
}
/*
#[derive(Debug)]
pub enum ServerError {
    //#[error(transparent)]
    ValidationError(validator::ValidationErrors),

    //#[error(transparent)]
    AxumJsonRejection(JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
} */