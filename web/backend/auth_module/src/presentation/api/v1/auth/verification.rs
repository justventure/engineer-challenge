use crate::presentation::api::v1::dto::auth::{
    SendCodeRequestDto, SubmitCodeRequestDto, VerifyByLinkRequestDto,
};
use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;
use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::account::verification::{
    SendVerificationCodeCommand, SubmitVerificationCodeCommand, VerifyByLinkCommand,
};
use rust_kratos::domain::value_objects::email::Email;

#[post("/verify/send_code")]
pub async fn send_verification_code(
    req: HttpRequest,
    body: web::Json<SendCodeRequestDto>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let email = Email::new(&body.email).map_err(|e| ApiError::validation(e.to_string()))?;
    let command = SendVerificationCodeCommand {
        request: rust_kratos::domain::ports::inbound::verification::SendCodeRequest {
            email,
            transient_payload: None,
        },
        cookie: session_cookie(&req),
    };

    let handler = &container.use_cases().commands.verification;
    CommandHandler::<SendVerificationCodeCommand>::handle(handler, command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/verify/submit_code")]
pub async fn submit_verification_code(
    req: HttpRequest,
    body: web::Json<SubmitCodeRequestDto>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let command = SubmitVerificationCodeCommand {
        request: rust_kratos::domain::ports::inbound::verification::SubmitCodeRequest {
            code: body.code.clone(),
            transient_payload: None,
        },
        cookie: session_cookie(&req).ok_or(ApiError::Unauthorized("Missing cookie".to_string()))?,
    };

    let handler = &container.use_cases().commands.verification;
    CommandHandler::<SubmitVerificationCodeCommand>::handle(handler, command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/verify/link")]
pub async fn verify_by_link(
    req: HttpRequest,
    body: web::Json<VerifyByLinkRequestDto>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let email = Email::new(&body.email).map_err(|e| ApiError::validation(e.to_string()))?;
    let command = VerifyByLinkCommand {
        request: rust_kratos::domain::ports::inbound::verification::VerifyByLinkRequest {
            email,
            transient_payload: None,
        },
        cookie: session_cookie(&req),
    };

    let handler = &container.use_cases().commands.verification;
    CommandHandler::<VerifyByLinkCommand>::handle(handler, command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().finish())
}
