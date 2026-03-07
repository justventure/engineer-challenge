use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::DomainError;
use crate::domain::ports::session::SessionPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use async_trait::async_trait;
use reqwest::header;
use std::sync::Arc;

pub struct KratosSessionAdapter {
    client: Arc<KratosClient>,
}

impl KratosSessionAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }

    pub async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, DomainError> {
        let url =
            format!("{}/sessions/whoami", self.client.public_url).replace("localhost", "127.0.0.1");

        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::NotAuthenticated);
        }

        let session_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        let email = session_data["identity"]["traits"]["email"]
            .as_str()
            .ok_or_else(|| DomainError::Unknown("Email not found".to_string()))?
            .to_string();

        let username = session_data["identity"]["traits"]["username"]
            .as_str()
            .ok_or_else(|| DomainError::Unknown("Username not found".to_string()))?
            .to_string();

        let geo_location = session_data["identity"]["traits"]["geo_location"]
            .as_str()
            .map(|s| s.to_string());

        Ok(UserProfile {
            email,
            username,
            geo_location,
        })
    }

    pub async fn is_recovery_session(&self, cookie: Option<&str>) -> bool {
        let Some(cookie) = cookie else { return false };

        let url =
            format!("{}/sessions/whoami", self.client.public_url).replace("localhost", "127.0.0.1");

        let Ok(response) = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
        else {
            return false;
        };

        let Ok(data) = response.json::<serde_json::Value>().await else {
            return false;
        };

        data["authentication_methods"]
            .as_array()
            .map(|methods| {
                methods.iter().any(|m| {
                    m["method"].as_str() == Some("link_recovery")
                        || m["method"].as_str() == Some("code_recovery")
                })
            })
            .unwrap_or(false)
    }

    async fn get_logout_flow(&self, cookie: &str) -> Result<String, DomainError> {
        let url = format!("{}/self-service/logout/browser", self.client.public_url)
            .replace("localhost", "127.0.0.1");

        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::Network(
                "Failed to get logout flow".to_string(),
            ));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        data["logout_url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| DomainError::Unknown("Logout URL not found".to_string()))
    }
}

#[async_trait]
impl SessionPort for KratosSessionAdapter {
    async fn logout(&self, cookie: &str) -> Result<(), DomainError> {
        let logout_url = self.get_logout_flow(cookie).await?;

        let response = self
            .client
            .client
            .get(&logout_url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        let status = response.status();
        if status.is_success() || status == 302 || status == 303 {
            return Ok(());
        }

        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        Err(DomainError::Unknown(format!(
            "Logout failed: {}",
            error_text
        )))
    }

    async fn check_active_session(&self, cookie: Option<&str>) -> bool {
        if let Some(cookie_value) = cookie {
            if self.get_current_user(cookie_value).await.is_ok() {
                return true;
            }
        }
        false
    }
}
