use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;
use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::queries::QueryHandler;
use rust_kratos::application::queries::get_current_user::GetCurrentUserQuery;

#[post("/me")]
pub async fn get_current_user(
    req: HttpRequest,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let query = GetCurrentUserQuery {
        cookie: session_cookie(&req),
    };

    let handler = &container.use_cases().queries.get_current_user;
    let user = handler.handle(query).await.map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().json(user))
}
