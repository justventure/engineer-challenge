pub mod auth;
pub mod errors;
pub mod extractors;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(auth::routes));
}
