use crate::presentation::api::v1::dto::auth::SettingsRequestDto;
use crate::presentation::api::v1::errors::ApiError;
use crate::presentation::api::v1::extractors::session_cookie;
use actix_web::{HttpRequest, HttpResponse, post, web};
use rust_kratos::AppContainer;
use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::account::settings::UpdateSettingsCommand;
use rust_kratos::domain::ports::inbound::settings::SettingsData;
use rust_kratos::domain::value_objects::password::Password;

#[post("/settings/update")]
pub async fn update_settings(
    req: HttpRequest,
    body: web::Json<SettingsRequestDto>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, ApiError> {
    let cookie =
        session_cookie(&req).ok_or(ApiError::Unauthorized("Missing cookie".to_string()))?;
    let data = SettingsData {
        method: body.method.clone(),
        password: body.password.as_ref().map(|p| Password::new(p).unwrap()),
        traits: body.traits.clone(),
        lookup_secret_confirm: body.lookup_secret_confirm,
        lookup_secret_disable: body.lookup_secret_disable,
        lookup_secret_regenerate: body.lookup_secret_regenerate,
        lookup_secret_reveal: body.lookup_secret_reveal,
        transient_payload: body.transient_payload.clone(),
    };

    let command = UpdateSettingsCommand { data, cookie };

    let handler = &container.use_cases().commands.update_settings;
    let result = handler
        .handle(command)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(HttpResponse::Ok().json(result))
}
