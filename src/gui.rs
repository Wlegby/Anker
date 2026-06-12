use crate::client::AnkiClient;
use crate::error::Result;
use serde::Serialize;

pub struct Gui<'a> {
    client: &'a AnkiClient,
}

impl<'a> Gui<'a> {
    pub(crate) fn new(client: &'a AnkiClient) -> Self {
        Self { client }
    }

    /// Opens the Anki browser with the specified query.
    pub async fn gui_browse(&self, query: &str) -> Result<Vec<i64>> {
        #[derive(Serialize)]
        struct Params<'a> {
            query: &'a str,
        }
        self.client
            .invoke("guiBrowse", Some(Params { query }))
            .await
    }

    /// Opens the Add dialog with an optional note payload.
    pub async fn gui_add_cards(&self, note: Option<&crate::notes::Note>) -> Result<i64> {
        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            note: Option<&'a crate::notes::Note>,
        }
        self.client
            .invoke("guiAddCards", Some(Params { note }))
            .await
    }

    /// Reviews the current card by answering with the given ease (1-4).
    pub async fn gui_answer_card(&self, ease: i32) -> Result<bool> {
        #[derive(Serialize)]
        struct Params {
            ease: i32,
        }
        self.client
            .invoke("guiAnswerCard", Some(Params { ease }))
            .await
    }

    /// Returns the ID of the current card in review.
    pub async fn gui_current_card(&self) -> Result<Option<i64>> {
        // May return null if not in review
        let res: Option<i64> = self
            .client
            .invoke::<(), Option<i64>>("guiCurrentCard", None)
            .await
            .unwrap_or(None);
        Ok(res)
    }
}

impl crate::AnkiClient {
    /// Access GUI-related actions.
    pub fn gui(&self) -> Gui<'_> {
        Gui::new(self)
    }
}
