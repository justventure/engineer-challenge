use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use rust_kratos::{AppContainer, ContainerConfig, KratosClientConfig};

use crate::presentation::api;
use crate::startup::config::Config;

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let container = AppContainer::new(ContainerConfig {
        kratos: KratosClientConfig {
            admin_url: cfg.kratos.admin_url.clone(),
            public_url: cfg.kratos.public_url.clone(),
            timeout_secs: cfg.kratos.timeout_secs,
            connect_timeout_secs: cfg.kratos.connect_timeout_secs,
            pool_idle_timeout_secs: cfg.kratos.pool_idle_timeout_secs,
            pool_max_idle_per_host: cfg.kratos.pool_max_idle_per_host,
            max_retries: cfg.kratos.max_retries,
            retry_delay_ms: cfg.kratos.retry_delay_ms,
            accept_invalid_certs: cfg.kratos.accept_invalid_certs,
            keep_alive_secs: cfg.kratos.keep_alive_secs,
            keep_alive_interval_secs: cfg.kratos.keep_alive_interval_secs,
        },
    })?;

    let container = web::Data::new(container);
    let bind_addr = format!("{}:{}", cfg.server.host, cfg.server.port);

    HttpServer::new(move || {
        let cors = build_cors(&cfg.server.cors_allowed_origins, cfg.server.cors_max_age);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(container.clone())
            .configure(api::routes)
    })
    .bind(&bind_addr)?
    .run()
    .await?;

    Ok(())
}

fn build_cors(origins: &[String], max_age: usize) -> Cors {
    if origins.iter().any(|o| o == "*") {
        return Cors::permissive();
    }

    let mut cors = Cors::default().max_age(max_age);
    for origin in origins {
        cors = cors.allowed_origin(origin);
    }
    cors
}
