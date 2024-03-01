use axum::{extract::Request, http::header, middleware::Next, response::{IntoResponse, Response}};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
//export CARGO_STATIC_DIR=/Users/gufei/Desktop/rust/mp/mp_server/static
#[folder = "/Users/gufei/Desktop/rust/mp/mp_server/static"]
//#[folder = "$CARGO_MANIFEST_DIR/static/"]
pub struct Asset;
pub async fn static_file(request: Request, next: Next) -> Response {
    //include_str!()
    let mut path = request.uri().path();
    path = &path[1..];
    if path == "" {
        path = "index.html";
    }
    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => next.run(request).await,
    }
}
