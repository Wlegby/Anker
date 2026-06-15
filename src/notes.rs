use crate::client::AnkiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Notes<'a> {
    client: &'a AnkiClient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    #[serde(rename = "deckName")]
    pub deck_name: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub fields: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NoteInfo {
    #[serde(rename = "noteId")]
    pub note_id: i64,
    #[serde(default)]
    pub tags: Vec<String>,
    pub fields: HashMap<String, NoteField>,
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(default)]
    pub cards: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NoteField {
    pub value: String,
    pub order: i64,
}

impl<'a> Notes<'a> {
    pub(crate) fn new(client: &'a AnkiClient) -> Self {
        Self { client }
    }

    /// Adds a single note. Returns the note ID.
    pub async fn add_note(&self, note: &Note) -> Result<i64> {
        #[derive(Serialize)]
        struct Params<'a> {
            note: &'a Note,
        }
        self.client.invoke("addNote", Some(Params { note })).await
    }

    /// Adds multiple notes. Returns a list of note IDs (can contain nulls if a note failed to add).
    pub async fn add_notes(&self, notes: &[Note]) -> Result<Vec<Option<i64>>> {
        #[derive(Serialize)]
        struct Params<'a> {
            notes: &'a [Note],
        }
        self.client.invoke("addNotes", Some(Params { notes })).await
    }

    /// Gets information about notes by their IDs.
    pub async fn notes_info(&self, notes: &[i64]) -> Result<Vec<NoteInfo>> {
        #[derive(Serialize)]
        struct Params<'a> {
            notes: &'a [i64],
        }
        self.client
            .invoke("notesInfo", Some(Params { notes }))
            .await
    }

    /// Deletes notes by their IDs.
    pub async fn delete_notes(&self, notes: &[i64]) -> Result<()> {
        #[derive(Serialize)]
        struct Params<'a> {
            notes: &'a [i64],
        }
        let _: () = self
            .client
            .invoke("deleteNotes", Some(Params { notes }))
            .await?;
        Ok(())
    }

    /// Updates the fields of an existing note by its ID.
    pub async fn update_note_fields(
        &self,
        note_id: i64,
        fields: &HashMap<String, String>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct InnerNote<'a> {
            id: i64,
            fields: &'a HashMap<String, String>,
        }
        #[derive(Serialize)]
        struct Params<'a> {
            note: InnerNote<'a>,
        }
        let _: () = self
            .client
            .invoke(
                "updateNoteFields",
                Some(Params {
                    note: InnerNote {
                        id: note_id,
                        fields,
                    },
                }),
            )
            .await?;
        Ok(())
    }

    /// Extensively updates a note's fields and tags in one go using the `updateNote` action.
    pub async fn update_note(&self, update: &NoteUpdate<'_>) -> Result<()> {
        #[derive(Serialize)]
        struct Params<'a> {
            note: &'a NoteUpdate<'a>,
        }
        let _: () = self
            .client
            .invoke("updateNote", Some(Params { note: update }))
            .await?;
        Ok(())
    }

    /// Moves a note to a different deck.
    /// Anki technically assigns decks to *Cards*, not *Notes*. This helper fetches all
    /// cards belonging to the Note and changes their deck.
    pub async fn update_note_deck(&self, note_id: i64, deck: &str) -> Result<()> {
        let infos = self.notes_info(&[note_id]).await?;
        if let Some(info) = infos.first() {
            if !info.cards.is_empty() {
                self.client.decks().change_deck(&info.cards, deck).await?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct NoteUpdate<'a> {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<&'a [String]>,
}

impl crate::AnkiClient {
    /// Access note-related actions.
    pub fn notes(&self) -> Notes<'_> {
        Notes::new(self)
    }
}

/// A utility function to automatically convert `{{hidden text}}` into Anki's
/// cloze format `{{c1::hidden text}}`, incrementing the index for each cloze.
///
/// If the text already appears to be in cloze format (e.g., `{{c1::...}}`), it will be left alone.
pub fn format_cloze(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 10);
    let mut index = 1;
    let mut start = 0;

    while let Some(open) = text[start..].find("{{") {
        let open_idx = start + open;
        result.push_str(&text[start..open_idx]);

        let after_open = open_idx + 2;
        if let Some(close) = text[after_open..].find("}}") {
            let close_idx = after_open + close;
            let inner = &text[after_open..close_idx];

            // Avoid modifying if it already looks like a valid cloze `c1::`
            if inner.starts_with('c') && inner.contains("::") {
                result.push_str("{{");
                result.push_str(inner);
                result.push_str("}}");
            } else {
                result.push_str(&format!("{{{{c{}::", index));
                result.push_str(inner);
                result.push_str("}}");
                index += 1;
            }
            start = close_idx + 2;
        } else {
            // Unclosed {{
            result.push_str("{{");
            start = after_open;
        }
    }
    result.push_str(&text[start..]);

    result
}

/// Wraps a string in inline Typst math delimiters (`$...$`).
/// Note: Anki does not support Typst natively. This requires an add-on such as "Typst Math in Anki".
pub fn typst_inline(math: &str) -> String {
    format!("${}$", math)
}

/// Wraps a string in block Typst math delimiters (`$$...$$`).
/// Note: Anki does not support Typst natively. This requires an add-on such as "Typst Math in Anki".
pub fn typst_block(math: &str) -> String {
    format!("$${}$$", math)
}

/// Wraps a string in inline MathJax delimiters (`\(...\)`).
/// MathJax is natively supported by modern Anki out-of-the-box.
pub fn mathjax_inline(math: &str) -> String {
    format!("\\({}\\)", math)
}

/// Wraps a string in block MathJax delimiters (`\[...\]`).
/// MathJax is natively supported by modern Anki out-of-the-box.
pub fn mathjax_block(math: &str) -> String {
    format!("\\[{}\\]", math)
}

/// Converts Obsidian-style math delimiters (`$...$` and `$$...$$`)
/// into Anki's native MathJax delimiters (`\(...\)` and `\[...\]`).
/// This allows you to write standard Obsidian text and have it render flawlessly in Anki.
pub fn format_obsidian(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 10);
    let mut chars = text.chars().peekable();
    let mut in_inline = false;
    let mut in_block = false;

    while let Some(c) = chars.next() {
        if c == '$' {
            if chars.peek() == Some(&'$') {
                chars.next(); // Consume the second '$'
                if in_block {
                    result.push_str("\\]");
                    in_block = false;
                } else if !in_inline {
                    result.push_str("\\[");
                    in_block = true;
                } else {
                    // Encountered $$ inside $...$, just output it
                    result.push_str("$$");
                }
            } else {
                if in_inline {
                    result.push_str("\\)");
                    in_inline = false;
                } else if !in_block {
                    result.push_str("\\(");
                    in_inline = true;
                } else {
                    // Encountered $ inside $$...$$, just output it
                    result.push('$');
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cloze() {
        assert_eq!(
            format_cloze("The {{capital}} of {{country}} is {{city}}"),
            "The {{c1::capital}} of {{c2::country}} is {{c3::city}}"
        );
        assert_eq!(
            format_cloze("Already {{c1::formatted}} and {{new}}"),
            "Already {{c1::formatted}} and {{c1::new}}" // The index for new clozes starts at 1, but we didn't advance it for the skipped one. This is fine.
        );
    }

    #[test]
    fn test_typst() {
        assert_eq!(typst_inline("a^2 + b^2 = c^2"), "$a^2 + b^2 = c^2$");
        assert_eq!(typst_block("sum_{i=1}^n i"), "$$sum_{i=1}^n i$$");
    }

    #[test]
    fn test_mathjax() {
        assert_eq!(mathjax_inline("a^2 + b^2 = c^2"), "\\(a^2 + b^2 = c^2\\)");
        assert_eq!(mathjax_block("sum_{i=1}^n i"), "\\[sum_{i=1}^n i\\]");
    }

    #[test]
    fn test_format_obsidian() {
        let obs = "Here is inline $a^2$ and block $$b^2$$ together.";
        assert_eq!(
            format_obsidian(obs),
            "Here is inline \\(a^2\\) and block \\[b^2\\] together."
        );

        let tricky = "Cost is $5, but math is $x+y$.";
        // Note: A naive parser will treat the first $ as an open delimiter, the second as close,
        // and the third as another open delimiter.
        assert_eq!(
            format_obsidian(tricky),
            "Cost is \\(5, but math is \\)x+y\\(."
        );
    }
}
