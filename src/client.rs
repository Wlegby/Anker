use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::error::{AnkiError, Result};

const DEFAULT_URL: &str = "http://127.0.0.1:8765";

#[derive(Debug, Clone)]
pub struct AnkiClient {
    client: Client,
    url: Url,
    version: u8,
}

#[derive(Serialize)]
struct AnkiRequest<'a, T> {
    action: &'a str,
    version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<T>,
}

#[derive(Deserialize)]
struct AnkiResponse<R> {
    result: Option<R>,
    error: Option<String>,
}

impl Default for AnkiClient {
    fn default() -> Self {
        Self::new(DEFAULT_URL)
    }
}

impl AnkiClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(url).unwrap_or_else(|_| Url::parse(DEFAULT_URL).unwrap()),
            version: 6,
        }
    }

    /// Invokes a raw AnkiConnect action.
    ///
    /// This is the low-level method. In most cases, you should use the typed
    /// methods in the other modules (e.g. `client.decks().deck_names()`).
    pub async fn invoke<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        action: &str,
        params: Option<T>,
    ) -> Result<R> {
        let req = AnkiRequest {
            action,
            version: self.version,
            params,
        };

        let response: AnkiResponse<R> = self
            .client
            .post(self.url.clone())
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        if let Some(err_msg) = response.error {
            return Err(AnkiError::Api(err_msg));
        }

        // According to AnkiConnect API, result could be null, but we usually expect it if no error
        // If the return type R is `()`, `serde_json` might deserialize `null` as `()`.
        match response.result {
            Some(res) => Ok(res),
            None => Err(AnkiError::Api(
                "Result is null but no error was provided".into(),
            )),
        }
    }
}
