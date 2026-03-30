use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;
use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::auth::logout::LogoutCommand;

#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let command = LogoutCommand {
        cookie: session_cookie(&req),
    };

    let handler = &container.use_cases().commands.logout;
    let _ = handler
        .handle(command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().finish())
}
