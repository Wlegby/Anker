# Anker

A simple to use, strongly-typed Rust library for interacting with [Anki](https://apps.ankiweb.net/) via the [AnkiConnect](https://foosoft.net/projects/anki-connect/) add-on.

## Features

- **Async API**: Built on top of `reqwest` for fast, non-blocking requests.
- **Typed Endpoints**: Wrappers for the most common Decks, Notes, Cards, Models, Media, and GUI actions.
- **Generic Fallback**: Easily invoke any AnkiConnect endpoint using the underlying `.invoke::<T, R>()` method.
- **System Launch**: Comes with a utility function to open the Anki app directly from your Rust code.

## Requirements

1. You must have [Anki](https://apps.ankiweb.net/) installed.
2. You must have the [AnkiConnect](https://foosoft.net/projects/anki-connect/) add-on installed (Code: `2055492159`).
3. Anki must be running in the background, or launched using `anker::launch_anki()`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anker = { path = "../anker" } # Or publish to crates.io
tokio = { version = "1.0", features = ["full"] }
```

## Examples

### 1. Adding a Basic Note

```rust
use anker::AnkiClient;
use anker::notes::Note;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnkiClient::default();
    
    let mut fields = HashMap::new();
    fields.insert("Front".to_string(), "What is Rust?".to_string());
    fields.insert("Back".to_string(), "A systems programming language.".to_string());

    let note = Note {
        deck_name: "Default".to_string(),
        model_name: "Basic".to_string(),
        fields,
        tags: vec!["programming".to_string(), "rust".to_string()],
    };

    let note_id = client.notes().add_note(&note).await?;
    println!("Added note with ID: {}", note_id);
    Ok(())
}
```

### 2. Adding a Cloze Note

Creating a Cloze card is as simple as using the `Cloze` model. You can manually use Anki's standard `{{c1::...}}` cloze deletion syntax, or you can use our `format_cloze` helper to automatically convert `{{...}}` tags!

```rust
use anker::AnkiClient;
use anker::notes::{Note, format_cloze};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnkiClient::default();

    let mut fields = HashMap::new();
    
    // format_cloze will convert this to: "Rust is a {{c1::systems}} programming {{c2::language}}."
    let raw_text = "Rust is a {{systems}} programming {{language}}.";
    fields.insert("Text".to_string(), format_cloze(raw_text));
    
    fields.insert("Back Extra".to_string(), "It's known for memory safety.".to_string());

    let note = Note {
        deck_name: "Default".to_string(),
        model_name: "Cloze".to_string(), // Crucial: must be the Cloze note type!
        fields,
        tags: vec!["rust".to_string(), "cloze".to_string()],
    };

    let note_id = client.notes().add_note(&note).await?;
    println!("Added cloze note with ID: {}", note_id);
    Ok(())
}
```

### 3. Uploading Media

You can upload media by passing a URL, a local path, or raw base64 data.

```rust
use anker::AnkiClient;
use anker::media::MediaSource;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnkiClient::default();

    // Store via URL
    let filename = client.media().store_media_file(MediaSource::Url {
        filename: "rust_logo.png",
        url: "https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg",
    }).await?;
    
    println!("Successfully stored media as: {}", filename);
    
    // You can now reference this media inside a Note field using `<img src="rust_logo.png">`
    Ok(())
}
```

### 4. Interacting with Note IDs

Once you add notes, you get a unique Note ID (an `i64`). You can fetch information about a note, update its fields, or delete it using this ID.

```rust
use anker::AnkiClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnkiClient::default();

    let note_id = 1234567890; // Replace with an actual Note ID

    // Fetch note info
    let infos = client.notes().notes_info(&[note_id]).await?;
    if let Some(info) = infos.first() {
        println!("Note {} has tags: {:?}", info.note_id, info.tags);
    }

    // Update note fields
    let mut updated_fields = HashMap::new();
    updated_fields.insert("Front".to_string(), "Updated Front text".to_string());
    client.notes().update_note_fields(note_id, &updated_fields).await?;
    println!("Note fields updated!");

    // Delete a note
    client.notes().delete_notes(&[note_id]).await?;
    println!("Note deleted!");

    Ok(())
}
```

## Modules overview

- `client.decks()`: Create, list, delete decks.
- `client.notes()`: Add, remove, info for notes.
- `client.cards()`: Search, find info, suspend/unsuspend cards.
- `client.models()`: Note types and fields.
- `client.media()`: Store and retrieve media files.
- `client.gui()`: Open browse windows, Add Card dialogs, review cards.

## Fallback: Generic Invoke

If an AnkiConnect endpoint is not yet supported in the typed wrappers, you can use the low-level `invoke` method.

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Params<'a> {
    query: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = anker::AnkiClient::default();
    
    let result: Vec<i64> = client
        .invoke("findCards", Some(Params { query: "deck:Default" }))
        .await?;
        
    Ok(())
}
```

## License

MIT
