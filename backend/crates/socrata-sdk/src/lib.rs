use reqwest::Client;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum SocrataError {
    #[error("API Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API Request returned status {0}: {1}")]
    ApiError(reqwest::StatusCode, String),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
}

/// A client for the Socrata Open Data API (SODA).
pub struct SocrataClient {
    client: Client,
    base_url: String,
    app_token: Option<String>,
}

impl SocrataClient {
    /// Creates a new `SocrataClient`.
    ///
    /// # Arguments
    /// * `base_url` - The base domain of the Socrata instance (e.g., "https://www.datos.gov.co")
    /// * `app_token` - Optional Socrata App Token for higher rate limits.
    pub fn new(base_url: &str, app_token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            app_token,
        }
    }

    /// Fetch data from a specific dataset ID using SODA API
    ///
    /// # Arguments
    /// * `dataset_id` - The 4x4 ID of the dataset (e.g. "jbjy-vk9h")
    /// * `limit` - Number of records to return
    /// * `offset` - Number of records to skip
    /// * `order` - SoQL order clause (e.g. "date DESC")
    /// * `where_clause` - Optional SoQL where clause
    pub async fn fetch<T: DeserializeOwned>(
        &self,
        dataset_id: &str,
        limit: u32,
        offset: u32,
        order: Option<&str>,
        where_clause: Option<&str>,
    ) -> Result<Vec<T>, SocrataError> {
        let mut url = format!(
            "{}/resource/{}.json?$limit={}&$offset={}",
            self.base_url, dataset_id, limit, offset
        );

        if let Some(ord) = order {
            url.push_str(&format!("&$order={}", ord));
        }

        if let Some(clause) = where_clause {
            url.push_str(&format!("&$where={}", clause));
        }

        info!("SODA Request: {}", url);

        let mut request = self.client.get(&url);

        if let Some(token) = &self.app_token {
            request = request.header("X-App-Token", token);
        } else {
            warn!("No App Token provided. Rate limits may apply.");
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SocrataError::ApiError(status, body));
        }

        let data: Vec<T> = response.json().await?;
        Ok(data)
    }
}
