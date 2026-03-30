pub mod logout;
pub mod me;
pub mod recovery;
pub mod registration;
pub mod settings;
pub mod verification;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(registration::registration)
        .service(recovery::recovery)
        .service(verification::send_verification_code)
        .service(verification::submit_verification_code)
        .service(verification::verify_by_link)
        .service(logout::logout)
        .service(settings::update_settings)
        .service(me::get_current_user);
}
