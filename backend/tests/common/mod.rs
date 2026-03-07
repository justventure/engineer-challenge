use reqwest::Client;
use rust_kratos::infrastructure::adapters::kratos::client::KratosClient;
use std::sync::Arc;
use std::time::Duration;

#[allow(dead_code)]
pub struct TestContext {
    pub client: Arc<KratosClient>,
}

#[allow(dead_code)]
impl TestContext {
    pub fn new() -> Self {
        let public_url = std::env::var("KRATOS_PUBLIC_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4433".to_string());
        let admin_url = std::env::var("KRATOS_ADMIN_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4434".to_string());

        Self {
            client: Arc::new(KratosClient {
                client: Client::builder()
                    .cookie_store(false)
                    .redirect(reqwest::redirect::Policy::none())
                    .danger_accept_invalid_certs(true)
                    .build()
                    .expect("Failed to build HTTP client"),
                public_url,
                admin_url,
                max_retries: 3,
                retry_delay: Duration::from_millis(1000),
            }),
        }
    }

    pub fn random_email() -> String {
        format!("test_{}@example.com", uuid::Uuid::new_v4())
    }
}
