use std::time::Duration;

use reqwest::{Client, Response, StatusCode, Url};
use tokio::time::sleep;

use crate::config::ResolvedConfig;
use crate::error::{AppError, AppResult};
use crate::okta::models::{Brand, EmailCustomization, EmailDefaultContent, EmailTemplate};

#[derive(Clone)]
pub struct OktaClient {
    client: Client,
    base_url: Url,
    token: String,
}

impl OktaClient {
    pub fn new(config: &ResolvedConfig) -> AppResult<Self> {
        let base_url = normalize_domain(&config.domain)?;
        let client = Client::builder()
            .user_agent("okta-template-downloader/0.1.0")
            .build()?;

        Ok(Self {
            client,
            base_url,
            token: config.token.clone(),
        })
    }

    pub async fn list_brands(&self) -> AppResult<Vec<Brand>> {
        self.get_paginated_json("/api/v1/brands?limit=200").await
    }

    pub async fn list_templates(&self, brand_id: &str) -> AppResult<Vec<EmailTemplate>> {
        self.get_paginated_json(&format!(
            "/api/v1/brands/{brand_id}/templates/email?limit=200"
        ))
        .await
    }

    pub async fn default_content(
        &self,
        brand_id: &str,
        template_name: &str,
    ) -> AppResult<EmailDefaultContent> {
        self.get_json(&format!(
            "/api/v1/brands/{brand_id}/templates/email/{template_name}/default-content"
        ))
        .await
    }

    pub async fn list_customizations(
        &self,
        brand_id: &str,
        template_name: &str,
    ) -> AppResult<Vec<EmailCustomization>> {
        self.get_paginated_json(&format!(
            "/api/v1/brands/{brand_id}/templates/email/{template_name}/customizations?limit=200"
        ))
        .await
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> AppResult<T> {
        let url = self.resolve_url(path)?;
        let response = self.send_with_retry(url).await?;
        Ok(response.json().await?)
    }

    async fn get_paginated_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> AppResult<Vec<T>> {
        let mut items = Vec::new();
        let mut next_url = Some(self.resolve_url(path)?);

        while let Some(url) = next_url {
            let response = self.send_with_retry(url).await?;
            next_url = next_link(response.headers());
            let mut page_items: Vec<T> = response.json().await?;
            items.append(&mut page_items);
        }

        Ok(items)
    }

    async fn send_with_retry(&self, url: Url) -> AppResult<Response> {
        let mut last_error = None;

        for attempt in 0..3 {
            let response = self
                .client
                .get(url.clone())
                .header(
                    reqwest::header::AUTHORIZATION,
                    format!("SSWS {}", self.token),
                )
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await?;

            if response.status() == StatusCode::TOO_MANY_REQUESTS {
                if attempt < 2 {
                    let retry_after = response
                        .headers()
                        .get(reqwest::header::RETRY_AFTER)
                        .and_then(|value| value.to_str().ok())
                        .and_then(|value| value.parse::<u64>().ok())
                        .unwrap_or(2);
                    sleep(Duration::from_secs(retry_after)).await;
                    continue;
                }
            }

            if response.status().is_server_error() && attempt < 2 {
                sleep(Duration::from_secs((attempt + 1) as u64)).await;
                continue;
            }

            if !response.status().is_success() {
                let status = response.status();
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "request failed".to_string());
                last_error = Some(AppError::Api { status, message });
                break;
            }

            return Ok(response);
        }

        Err(last_error.unwrap_or_else(|| AppError::Api {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "request failed after retries".to_string(),
        }))
    }

    fn resolve_url(&self, path: &str) -> AppResult<Url> {
        self.base_url
            .join(path.trim_start_matches('/'))
            .map_err(|_| AppError::InvalidDomain(self.base_url.to_string()))
    }
}

fn normalize_domain(input: &str) -> AppResult<Url> {
    let normalized = if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else {
        format!("https://{input}")
    };

    let mut url =
        Url::parse(&normalized).map_err(|_| AppError::InvalidDomain(input.to_string()))?;
    if !url.path().ends_with('/') {
        url.set_path("/");
    }

    Ok(url)
}

fn next_link(headers: &reqwest::header::HeaderMap) -> Option<Url> {
    let header_value = headers.get(reqwest::header::LINK)?.to_str().ok()?;

    for part in header_value.split(',') {
        let section = part.trim();
        if !section.contains("rel=\"next\"") {
            continue;
        }

        let start = section.find('<')?;
        let end = section[start + 1..].find('>')?;
        let url = &section[start + 1..start + 1 + end];
        if let Ok(parsed) = Url::parse(url) {
            return Some(parsed);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, HeaderValue, LINK};

    use super::{next_link, normalize_domain};

    #[test]
    fn normalize_domain_adds_https() {
        let url = normalize_domain("example.okta.com").expect("domain should parse");
        assert_eq!(url.as_str(), "https://example.okta.com/");
    }

    #[test]
    fn extracts_next_link_from_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            LINK,
            HeaderValue::from_static(
                "<https://example.okta.com/api/v1/brands?limit=200>; rel=\"self\", <https://example.okta.com/api/v1/brands?limit=200&after=abc>; rel=\"next\"",
            ),
        );

        let next = next_link(&headers).expect("next link should parse");
        assert_eq!(
            next.as_str(),
            "https://example.okta.com/api/v1/brands?limit=200&after=abc"
        );
    }
}
