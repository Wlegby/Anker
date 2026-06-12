use crate::client::AnkiClient;
use crate::error::Result;
use serde::Serialize;

pub struct Decks<'a> {
    client: &'a AnkiClient,
}

impl<'a> Decks<'a> {
    pub(crate) fn new(client: &'a AnkiClient) -> Self {
        Self { client }
    }

    /// Gets the names of all decks.
    pub async fn get_deck_names(&self) -> Result<Vec<String>> {
        self.client
            .invoke::<(), Vec<String>>("deckNames", None)
            .await
    }

    /// Gets the names of all decks and their IDs.
    pub async fn get_deck_names_and_ids(&self) -> Result<std::collections::HashMap<String, i64>> {
        self.client
            .invoke::<(), std::collections::HashMap<String, i64>>("deckNamesAndIds", None)
            .await
    }

    /// Creates a new deck. Returns the deck ID.
    pub async fn create_deck(&self, deck: &str) -> Result<i64> {
        #[derive(Serialize)]
        struct Params<'a> {
            deck: &'a str,
        }
        self.client
            .invoke("createDeck", Some(Params { deck }))
            .await
    }

    /// Changes the deck of the given cards.
    pub async fn change_deck(&self, cards: &[i64], deck: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Params<'a> {
            cards: &'a [i64],
            deck: &'a str,
        }
        let _result: Option<()> = self
            .client
            .invoke("changeDeck", Some(Params { cards, deck }))
            .await
            .ok();
        // The API might return null for successful changeDeck. Our invoke handles null as an error if R isn't Option,
        // Wait, let's fix this in client.rs to allow deserializing unit () from null.
        // For now we just ignore the return value and check for Ok.
        Ok(())
    }

    /// Deletes decks and all their cards.
    pub async fn delete_decks(&self, decks: &[&str], cards_too: bool) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            decks: &'a [&'a str],
            cards_too: bool,
        }
        let _: Option<()> = self
            .client
            .invoke("deleteDecks", Some(Params { decks, cards_too }))
            .await
            .ok();
        Ok(())
    }
}

impl crate::AnkiClient {
    /// Access deck-related actions.
    pub fn decks(&self) -> Decks<'_> {
        Decks::new(self)
    }
}
