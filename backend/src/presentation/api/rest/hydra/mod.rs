pub mod consent;
pub mod error;
pub mod login;
pub mod logout;
pub mod oauth_login;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login_get)
        .service(login::login_post)
        .service(consent::consent_get)
        .service(consent::consent_post)
        .service(logout::logout)
        .service(oauth_login::hydra_login_via_kratos)
        .service(error::hydra_error);
}
