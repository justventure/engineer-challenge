pub mod login;
pub mod registration;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login)
        .service(registration::registration);
}
