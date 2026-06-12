use crate::client::AnkiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub struct Cards<'a> {
    client: &'a AnkiClient,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CardInfo {
    #[serde(rename = "cardId")]
    pub card_id: i64,
    pub fields: std::collections::HashMap<String, CardField>,
    pub field_order: i64,
    pub question: String,
    pub answer: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub ord: i64,
    #[serde(rename = "deckName")]
    pub deck_name: String,
    #[serde(rename = "css")]
    pub css: String,
    pub factor: i64,
    pub interval: i64,
    #[serde(rename = "note")]
    pub note_id: i64,
    #[serde(rename = "type")]
    pub card_type: i64,
    pub queue: i64,
    pub due: i64,
    pub reps: i64,
    pub lapses: i64,
    pub left: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CardField {
    pub value: String,
    pub order: i64,
}

impl<'a> Cards<'a> {
    pub(crate) fn new(client: &'a AnkiClient) -> Self {
        Self { client }
    }

    /// Finds cards using Anki's search syntax. Returns a list of card IDs.
    pub async fn find_cards(&self, query: &str) -> Result<Vec<i64>> {
        #[derive(Serialize)]
        struct Params<'a> {
            query: &'a str,
        }
        self.client
            .invoke("findCards", Some(Params { query }))
            .await
    }

    /// Gets information about cards by their IDs.
    pub async fn cards_info(&self, cards: &[i64]) -> Result<Vec<CardInfo>> {
        #[derive(Serialize)]
        struct Params<'a> {
            cards: &'a [i64],
        }
        self.client
            .invoke("cardsInfo", Some(Params { cards }))
            .await
    }

    /// Suspends the given cards.
    pub async fn suspend(&self, cards: &[i64]) -> Result<bool> {
        #[derive(Serialize)]
        struct Params<'a> {
            cards: &'a [i64],
        }
        self.client.invoke("suspend", Some(Params { cards })).await
    }

    /// Unsuspends the given cards.
    pub async fn unsuspend(&self, cards: &[i64]) -> Result<bool> {
        #[derive(Serialize)]
        struct Params<'a> {
            cards: &'a [i64],
        }
        self.client
            .invoke("unsuspend", Some(Params { cards }))
            .await
    }
}

impl crate::AnkiClient {
    /// Access card-related actions.
    pub fn cards(&self) -> Cards<'_> {
        Cards::new(self)
    }
}
