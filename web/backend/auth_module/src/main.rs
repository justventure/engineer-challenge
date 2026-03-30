use auth_module::startup::init;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init::run().await
}
