use maud::Markup;
use warp::http::{Response, StatusCode};

pub fn hx_with(
    status: StatusCode,
    res: Markup,
) -> Response<String> {
    Response::builder()
        .header("Content", "text/html")
        .status(status)
        .body(res.into_string())
        // TODO: verify that this is the case
        .expect("BUG: impossible")
}

pub fn hx(res: Markup) -> Response<String> {
    hx_with(StatusCode::OK, res)
}
pub fn hx_nc(res: Markup) -> Response<String> {
    hx_with(StatusCode::NO_CONTENT, res)
}
pub fn hx_notfound(res: Markup) -> Response<String> {
    hx_with(StatusCode::NOT_FOUND, res)
}
pub fn hx_badrequest(res: Markup) -> Response<String> {
    hx_with(StatusCode::BAD_REQUEST, res)
}
pub fn hx_forbidden(res: Markup) -> Response<String> {
    hx_with(StatusCode::FORBIDDEN, res)
}
