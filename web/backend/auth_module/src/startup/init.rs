use crate::infrastructure::http::server;
use crate::startup::config::Config;

pub async fn run() -> anyhow::Result<()> {
    let cfg = Config::from_env()?;
    server::run(cfg).await
}
