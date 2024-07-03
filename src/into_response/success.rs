use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

pub struct Success<T>(pub T);
impl<T> IntoResponse for Success<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let json_value = json!({
            "status":true,
            "message":"操作成功",
            "data":self.0,
        });
        (
            StatusCode::OK,
            [("content-type", "application/json")],
            json_value.to_string(),
        )
            .into_response()
    }
}
