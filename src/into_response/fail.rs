use axum::{response::IntoResponse, http::StatusCode};


pub struct Fail(pub &'static str);
impl IntoResponse for Fail
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}