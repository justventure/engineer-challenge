use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::identity::register::RegisterCommand;
use rust_kratos::domain::ports::inbound::registration::RegistrationData;
use rust_kratos::domain::value_objects::email::Email;
use rust_kratos::domain::value_objects::password::Password;
use serde::{Deserialize, Serialize};

use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub flow_id: String,
    pub user_id: String,
    pub email: String,
    pub session_cookie: String,
}

#[post("/auth/register")]
pub async fn registration(
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let command = RegisterCommand {
        data: RegistrationData {
            email: Email::new(&body.email).map_err(|e| ApiError::validation(e.to_string()))?,
            username: body.username.clone(),
            password: Password::new(&body.password)
                .map_err(|e| ApiError::validation(e.to_string()))?,
            geo_location: body.geo_location.clone(),
        },
        cookie: session_cookie(&req),
    };

    let result = container
        .use_cases()
        .commands
        .register
        .handle(command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Created().json(RegisterResponse {
        flow_id: result.flow_id,
        user_id: result.user.id.to_string(),
        email: result.user.traits.email.to_string(),
        session_cookie: result.session_cookie,
    }))
}
