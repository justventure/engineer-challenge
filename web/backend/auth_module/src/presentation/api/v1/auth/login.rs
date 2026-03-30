use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::auth::login::LoginCommand;
use rust_kratos::domain::ports::inbound::login::LoginCredentials;
use rust_kratos::domain::value_objects::email::Email;
use rust_kratos::domain::value_objects::password::Password;
use serde::{Deserialize, Serialize};

use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub email: String,
    pub session_cookie: String,
}

#[post("/auth/login")]
pub async fn login(
    req: HttpRequest,
    body: web::Json<LoginRequest>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let command = LoginCommand {
        credentials: LoginCredentials {
            identifier: Email::new(&body.email).map_err(|e| ApiError::validation(e.to_string()))?,
            password: Password::new(&body.password)
                .map_err(|e| ApiError::validation(e.to_string()))?,
            address: None,
            code: None,
            resend: None,
        },
        cookie: session_cookie(&req),
    };

    let result = container
        .use_cases()
        .commands
        .login
        .handle(command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        user_id: result.user.id.to_string(),
        email: result.user.traits.email.to_string(),
        session_cookie: result.session_cookie,
    }))
}
