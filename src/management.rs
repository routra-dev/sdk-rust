//! Management API client for Routra.
//!
//! Typed methods for all Routra management endpoints: keys, policies,
//! usage, billing, batch, webhooks, BYOK, notifications, providers.
//!
//! Access via [`Routra::management()`](crate::Routra::management) or
//! construct directly:
//!
//! ```no_run
//! use routra::management::ManagementClient;
//!
//! let mgmt = ManagementClient::new("rtr-...", "https://api.routra.dev");
//! let keys = mgmt.list_keys().await?;
//! ```

use reqwest::{header, Client, Method};
use serde::{Deserialize, Serialize};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CreateKeyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_rpm: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_rpd: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateKeyResponse {
    pub id: String,
    pub key: String,
    pub prefix: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeySummary {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub policy_id: Option<String>,
    pub rate_limit_rpm: Option<u32>,
    pub rate_limit_rpd: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub strategy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyResponse {
    pub id: String,
    pub name: String,
    pub strategy: String,
    pub constraints: serde_json::Value,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModalityUsage {
    pub usage_unit: String,
    pub request_count: u64,
    pub total_cost_usd: f64,
    pub total_usage_value: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsageSummary {
    pub total_requests: u64,
    pub total_cost_usd: f64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub period_start: String,
    pub period_end: String,
    #[serde(default)]
    pub modality_breakdown: Vec<ModalityUsage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CostBreakdownItem {
    pub model: String,
    pub provider: String,
    pub request_count: u64,
    pub total_cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestLogEntry {
    pub id: String,
    pub model: String,
    pub provider: String,
    pub latency_ms: u32,
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub created_at: String,
    pub usage_unit: Option<String>,
    pub usage_value: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BillingInfo {
    pub billing_tier: String,
    pub credit_balance_usd: f64,
    pub monthly_spend_usd: f64,
    pub subscription_status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCheckoutRequest {
    pub plan: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCheckoutResponse {
    pub checkout_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TopupRequest {
    pub amount_usd: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBatchRequest {
    pub requests: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchJobResponse {
    pub id: String,
    pub status: String,
    pub total_requests: u64,
    pub completed_requests: u64,
    pub failed_requests: u64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEndpointResponse {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoreKeyRequest {
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StoredKeyInfo {
    pub provider_slug: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyKeyResponse {
    pub valid: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationPreference {
    pub event_type: String,
    pub email_enabled: bool,
    pub webhook_enabled: bool,
    pub in_app_enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePreferenceRequest {
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InboxItem {
    pub id: String,
    pub event_type: String,
    pub title: String,
    pub body: String,
    pub read: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnreadCount {
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderInfo {
    pub slug: String,
    pub name: String,
    pub is_healthy: bool,
    #[serde(default)]
    pub supported_modalities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProvidersResponse {
    pub providers: Vec<ProviderInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub action: String,
    pub actor: String,
    pub resource_type: String,
    pub resource_id: String,
    pub created_at: String,
    pub details: Option<serde_json::Value>,
}

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ManagementError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Routra API {method} {path} failed ({status}): {body}")]
    Api {
        status: u16,
        body: String,
        method: String,
        path: String,
    },
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Typed client for the Routra Management API.
pub struct ManagementClient {
    client: Client,
    api_key: String,
    /// Base URL without trailing `/v1`, e.g. `https://api.routra.dev`
    base_url: String,
}

impl ManagementClient {
    /// Create a new management client.
    ///
    /// `base_url` should be the root (e.g. `https://api.routra.dev`).
    /// `/v1` is appended automatically.
    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> Result<T, ManagementError> {
        let url = format!("{}/v1{}", self.base_url, path);
        let resp = self
            .client
            .request(method.clone(), &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ManagementError::Api {
                status,
                body,
                method: method.to_string(),
                path: path.to_string(),
            });
        }
        Ok(resp.json().await?)
    }

    async fn request_with_body<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T, ManagementError> {
        let url = format!("{}/v1{}", self.base_url, path);
        let resp = self
            .client
            .request(method.clone(), &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body_text = resp.text().await.unwrap_or_default();
            return Err(ManagementError::Api {
                status,
                body: body_text,
                method: method.to_string(),
                path: path.to_string(),
            });
        }
        Ok(resp.json().await?)
    }

    async fn request_no_body(
        &self,
        method: Method,
        path: &str,
    ) -> Result<(), ManagementError> {
        let url = format!("{}/v1{}", self.base_url, path);
        let resp = self
            .client
            .request(method.clone(), &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ManagementError::Api {
                status,
                body,
                method: method.to_string(),
                path: path.to_string(),
            });
        }
        Ok(())
    }

    async fn request_with_body_no_response(
        &self,
        method: Method,
        path: &str,
        body: &impl Serialize,
    ) -> Result<(), ManagementError> {
        let url = format!("{}/v1{}", self.base_url, path);
        let resp = self
            .client
            .request(method.clone(), &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body_text = resp.text().await.unwrap_or_default();
            return Err(ManagementError::Api {
                status,
                body: body_text,
                method: method.to_string(),
                path: path.to_string(),
            });
        }
        Ok(())
    }

    // ── Keys ──────────────────────────────────────────────────────────────

    pub async fn create_key(&self, req: CreateKeyRequest) -> Result<CreateKeyResponse, ManagementError> {
        self.request_with_body(Method::POST, "/keys", &req).await
    }

    pub async fn list_keys(&self) -> Result<Vec<KeySummary>, ManagementError> {
        self.request(Method::GET, "/keys").await
    }

    pub async fn revoke_key(&self, id: &str) -> Result<(), ManagementError> {
        self.request_no_body(Method::DELETE, &format!("/keys/{id}")).await
    }

    pub async fn rotate_key(&self, id: &str) -> Result<CreateKeyResponse, ManagementError> {
        self.request_with_body(Method::POST, &format!("/keys/{id}/rotate"), &()).await
    }

    // ── Policies ──────────────────────────────────────────────────────────

    pub async fn create_policy(&self, req: CreatePolicyRequest) -> Result<PolicyResponse, ManagementError> {
        self.request_with_body(Method::POST, "/policies", &req).await
    }

    pub async fn list_policies(&self) -> Result<Vec<PolicyResponse>, ManagementError> {
        self.request(Method::GET, "/policies").await
    }

    // ── Usage ─────────────────────────────────────────────────────────────

    pub async fn usage(&self) -> Result<UsageSummary, ManagementError> {
        self.request(Method::GET, "/usage").await
    }

    pub async fn cost_breakdown(&self) -> Result<Vec<CostBreakdownItem>, ManagementError> {
        self.request(Method::GET, "/usage/cost-breakdown").await
    }

    pub async fn list_requests(&self, limit: u32, offset: u32) -> Result<Vec<RequestLogEntry>, ManagementError> {
        self.request(Method::GET, &format!("/requests?limit={limit}&offset={offset}")).await
    }

    // ── Billing ───────────────────────────────────────────────────────────

    pub async fn billing(&self) -> Result<BillingInfo, ManagementError> {
        self.request(Method::GET, "/billing").await
    }

    pub async fn create_checkout(&self, req: CreateCheckoutRequest) -> Result<CreateCheckoutResponse, ManagementError> {
        self.request_with_body(Method::POST, "/billing/checkout", &req).await
    }

    pub async fn cancel_subscription(&self) -> Result<(), ManagementError> {
        self.request_no_body(Method::DELETE, "/billing/subscription").await
    }

    pub async fn topup(&self, req: TopupRequest) -> Result<CreateCheckoutResponse, ManagementError> {
        self.request_with_body(Method::POST, "/billing/topup", &req).await
    }

    // ── Batch ─────────────────────────────────────────────────────────────

    pub async fn create_batch(&self, req: CreateBatchRequest) -> Result<BatchJobResponse, ManagementError> {
        self.request_with_body(Method::POST, "/batch", &req).await
    }

    pub async fn list_batches(&self) -> Result<Vec<BatchJobResponse>, ManagementError> {
        self.request(Method::GET, "/batch").await
    }

    pub async fn batch_status(&self, id: &str) -> Result<BatchJobResponse, ManagementError> {
        self.request(Method::GET, &format!("/batch/{id}/status")).await
    }

    pub async fn batch_results(&self, id: &str) -> Result<serde_json::Value, ManagementError> {
        self.request(Method::GET, &format!("/batch/{id}/results")).await
    }

    pub async fn cancel_batch(&self, id: &str) -> Result<(), ManagementError> {
        self.request_no_body(Method::POST, &format!("/batch/{id}/cancel")).await
    }

    // ── Webhooks ──────────────────────────────────────────────────────────

    pub async fn create_webhook(&self, req: CreateWebhookRequest) -> Result<WebhookEndpointResponse, ManagementError> {
        self.request_with_body(Method::POST, "/webhooks", &req).await
    }

    pub async fn list_webhooks(&self) -> Result<Vec<WebhookEndpointResponse>, ManagementError> {
        self.request(Method::GET, "/webhooks").await
    }

    pub async fn delete_webhook(&self, id: &str) -> Result<(), ManagementError> {
        self.request_no_body(Method::DELETE, &format!("/webhooks/{id}")).await
    }

    // ── BYOK (Provider Keys) ─────────────────────────────────────────────

    pub async fn store_provider_key(&self, provider_slug: &str, req: StoreKeyRequest) -> Result<(), ManagementError> {
        self.request_with_body_no_response(Method::POST, &format!("/provider-keys/{provider_slug}"), &req).await
    }

    pub async fn list_provider_keys(&self) -> Result<Vec<StoredKeyInfo>, ManagementError> {
        self.request(Method::GET, "/provider-keys").await
    }

    pub async fn delete_provider_key(&self, provider_slug: &str) -> Result<(), ManagementError> {
        self.request_no_body(Method::DELETE, &format!("/provider-keys/{provider_slug}")).await
    }

    pub async fn verify_provider_key(&self, provider_slug: &str) -> Result<VerifyKeyResponse, ManagementError> {
        self.request_with_body(Method::POST, &format!("/provider-keys/{provider_slug}/verify"), &()).await
    }

    // ── Notifications ─────────────────────────────────────────────────────

    pub async fn list_notification_preferences(&self) -> Result<Vec<NotificationPreference>, ManagementError> {
        self.request(Method::GET, "/notifications/preferences").await
    }

    pub async fn update_notification_preference(&self, req: UpdatePreferenceRequest) -> Result<(), ManagementError> {
        self.request_with_body_no_response(Method::PUT, "/notifications/preferences", &req).await
    }

    pub async fn list_inbox(&self, limit: u32, offset: u32) -> Result<Vec<InboxItem>, ManagementError> {
        self.request(Method::GET, &format!("/notifications/inbox?limit={limit}&offset={offset}")).await
    }

    pub async fn mark_read(&self, id: &str) -> Result<(), ManagementError> {
        self.request_no_body(Method::POST, &format!("/notifications/inbox/{id}/read")).await
    }

    pub async fn mark_all_read(&self) -> Result<(), ManagementError> {
        self.request_no_body(Method::POST, "/notifications/inbox/read-all").await
    }

    pub async fn unread_count(&self) -> Result<UnreadCount, ManagementError> {
        self.request(Method::GET, "/notifications/inbox/unread-count").await
    }

    // ── Providers ─────────────────────────────────────────────────────────

    pub async fn list_providers(&self) -> Result<ProvidersResponse, ManagementError> {
        self.request(Method::GET, "/providers").await
    }

    pub async fn catalog(&self) -> Result<serde_json::Value, ManagementError> {
        self.request(Method::GET, "/models/catalog").await
    }

    // ── Audit Log ─────────────────────────────────────────────────────────

    pub async fn list_audit_log(&self, limit: u32, offset: u32) -> Result<Vec<AuditLogEntry>, ManagementError> {
        self.request(Method::GET, &format!("/audit-log?limit={limit}&offset={offset}")).await
    }
}
