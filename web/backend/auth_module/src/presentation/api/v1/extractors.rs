use actix_web::HttpRequest;

pub fn session_cookie(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}
