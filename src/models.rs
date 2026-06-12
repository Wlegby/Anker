use crate::client::AnkiClient;
use crate::error::Result;
use serde::Serialize;

pub struct Models<'a> {
    client: &'a AnkiClient,
}

impl<'a> Models<'a> {
    pub(crate) fn new(client: &'a AnkiClient) -> Self {
        Self { client }
    }

    /// Gets the names of all models (note types).
    pub async fn model_names(&self) -> Result<Vec<String>> {
        self.client
            .invoke::<(), Vec<String>>("modelNames", None)
            .await
    }

    /// Gets the field names for a specific model.
    pub async fn model_field_names(&self, model_name: &str) -> Result<Vec<String>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            model_name: &'a str,
        }
        self.client
            .invoke("modelFieldNames", Some(Params { model_name }))
            .await
    }
}

impl crate::AnkiClient {
    /// Access model-related actions.
    pub fn models(&self) -> Models<'_> {
        Models::new(self)
    }
}
