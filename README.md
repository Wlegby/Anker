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

### 1. Adding a Basic (or Reversed) Note

You can manually construct a `Note`, or use the convenience constructor for standard Anki models like `Basic`. By passing `true` as the last argument, it automatically uses the `"Basic (and reversed card)"` model to generate two-way flashcards!

```rust
use anker::AnkiClient;
use anker::notes::Note;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnkiClient::default();

    // Standard "Basic" note (Front -> Back)
    let basic_note = Note::basic(
        "Default", 
        "What is Rust?", 
        "A systems programming language.",
        false // Not reversed
    ).with_tag("programming").with_tag("rust");

    let note_id = client.notes().add_note(&basic_note).await?;
    println!("Added basic note with ID: {}", note_id);

    // Standard "Basic (and reversed card)" note (Front <-> Back)
    let reversed_note = Note::basic(
        "Default",
        "el perro",
        "the dog",
        true // Reversed!
    ).with_tag("spanish");

    let reversed_id = client.notes().add_note(&reversed_note).await?;
    println!("Added reversed note with ID: {}", reversed_id);
    
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

Once you add notes, you get a unique Note ID (an `i64`). You can fetch information about a note, update its fields, tags, decks, or delete it using this ID.

```rust
use anker::AnkiClient;
use std::collections::HashMap;
use anker::notes::NoteUpdate;

#[tokio::main]
async fn main() -> anker::Result<()> {
    let client = anker::AnkiClient::new();
    let note_id = 1600000000000; // The ID of your note

    // 1. Fetch info
    let infos = client.notes().notes_info(&[note_id]).await?;

    // 2. Update ONLY the fields
    let mut fields = HashMap::new();
    fields.insert("Front".to_string(), "Updated front!".to_string());
    client.notes().update_note_fields(note_id, &fields).await?;

    // 3. Extensively update fields and tags in one go
    let new_tags = vec!["updated-tag".to_string()];
    let update = NoteUpdate {
        id: note_id,
        fields: Some(&fields), // Optional
        tags: Some(&new_tags), // Optional
    };
    client.notes().update_note(&update).await?;
    
    // 4. Move the note to a different deck
    // Note: This magically finds all cards belonging to the note and moves them for you!
    client.notes().update_note_deck(note_id, "Languages::Rust").await?;

    Ok(())
}
```

### 5. Handling Math and Markdown (Obsidian to Anki)

If you write your notes in **Obsidian** (using Markdown for bold, italics, lists, and `$` or `$$` for math), you can seamlessly convert it into Anki-ready HTML using our built-in Markdown compiler! 

```rust
use anker::markdown::markdown_to_anki;

let obsidian_text = "Here is **bold**, *italic*, and an equation: $$a^2 + b^2 = c^2$$";

// Automatically converts to: 
// "<p>Here is <strong>bold</strong>, <em>italic</em>, and an equation: \\[a^2 + b^2 = c^2\\]</p>"
let html_for_anki = markdown_to_anki(obsidian_text);
```

#### Transpiling Typst Math to Anki MathJax using Pandoc

If you are using Obsidian with a **Typst** plugin (writing Typst code inside your `$...$` blocks like `$sum_(i=1)^n i$`), you can use our advanced `markdown_to_anki_with_typst` function. 

This will parse your Markdown, extract your Typst math, and use **Pandoc** under the hood to compile the Typst syntax directly into Anki's native LaTeX/MathJax! *(Requires `pandoc` to be installed on your system).*

```rust
use anker::markdown::markdown_to_anki_with_typst;

let obsidian_typst = "Compute the sum: $sum_(i=1)^n i$";

// Pandoc compiles the Typst math into LaTeX on the fly!
// Outputs: "<p>Compute the sum: \\(\\sum_{i = 1}^{n}i\\)</p>"
let html_for_anki = markdown_to_anki_with_typst(obsidian_typst);
```

#### Automatically Uploading Local Obsidian Images

If your Markdown contains Obsidian image wikilinks with absolute paths (e.g., `[[/home/user/Pictures/image.png]]`), `anker` can automatically upload these files to Anki and replace the links with valid HTML `<img src="anki_filename.png">` tags!

```rust
use anker::AnkiClient;
use anker::markdown::upload_media;

#[tokio::main]
async fn main() -> anker::Result<()> {
    let client = AnkiClient::new();
    let text = "Here is my image: [[/absolute/path/to/my_image.png|Size 200x200]]";
    
    // Scans text, detects the absolute path, securely uploads it to Anki, 
    // and returns the rewritten string: "Here is my image: <img src=\"my_image.png\">"
    let updated_text = upload_media(&client, text).await?;
    
    // Now you can safely compile it to HTML!
    let html = anker::markdown::markdown_to_anki(&updated_text);
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
