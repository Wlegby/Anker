use crate::client::AnkiClient;
use crate::error::Result;
use serde::Serialize;

pub struct Media<'a> {
    client: &'a AnkiClient,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MediaSource<'a> {
    /// Provide base64 encoded data directly
    Data { filename: &'a str, data: &'a str },
    /// Provide a local filesystem path
    Path { filename: &'a str, path: &'a str },
    /// Provide a URL for AnkiConnect to download from
    Url { filename: &'a str, url: &'a str },
}

impl<'a> Media<'a> {
    pub(crate) fn new(client: &'a AnkiClient) -> Self {
        Self { client }
    }

    /// Stores a media file in the Anki collection. Returns the actual filename saved (might be renamed if conflict).
    pub async fn store_media_file(&self, source: MediaSource<'_>) -> Result<String> {
        self.client.invoke("storeMediaFile", Some(source)).await
    }

    /// Retrieves the base64 encoded data of a media file.
    /// Returns `Ok(Some(String))` if the file exists, or `Ok(None)` if the file does not exist.
    pub async fn retrieve_media_file(&self, filename: &str) -> Result<Option<String>> {
        #[derive(Serialize)]
        struct Params<'a> {
            filename: &'a str,
        }

        let val: serde_json::Value = self
            .client
            .invoke("retrieveMediaFile", Some(Params { filename }))
            .await?;

        match val {
            serde_json::Value::String(s) => Ok(Some(s)),
            serde_json::Value::Bool(false) => Ok(None),
            _ => Err(crate::error::AnkiError::Api(
                "Unexpected return type from retrieveMediaFile".into(),
            )),
        }
    }
}

impl crate::AnkiClient {
    /// Access media-related actions.
    pub fn media(&self) -> Media<'_> {
        Media::new(self)
    }
}
