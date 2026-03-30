use crate::presentation::api::v1::dto::auth::RecoveryRequestDto;
use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;
use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::account::recovery::RecoveryCommand;
use rust_kratos::domain::value_objects::email::Email;

#[post("/recovery")]
pub async fn recovery(
    req: HttpRequest,
    body: web::Json<RecoveryRequestDto>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let email = Email::new(&body.email).map_err(|e| ApiError::validation(e.to_string()))?;
    let command = RecoveryCommand {
        request: rust_kratos::domain::ports::inbound::recovery::RecoveryRequest { email },
        cookie: session_cookie(&req),
    };

    let handler = &container.use_cases().commands.recovery;
    handler
        .handle(command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().finish())
}
