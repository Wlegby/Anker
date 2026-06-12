use anker::notes::Note;
use anker::{AnkiClient, launch_anki};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. (Optional) Open Anki if it's not already open
    println!("Launching Anki...");
    launch_anki()?;
    std::thread::sleep(std::time::Duration::from_secs(5)); // Wait for Anki to load

    let client = AnkiClient::default();

    println!("Fetching deck names...");
    let decks = client.decks().get_deck_names().await?;
    println!("Available Decks: {:?}", decks);

    println!("Creating a new Note payload...");
    let mut fields = HashMap::new();
    fields.insert("Front".to_string(), "What is Rust?".to_string());
    fields.insert(
        "Back".to_string(),
        "A fast and safe systems programming language.".to_string(),
    );

    let note = Note {
        deck_name: "Default".to_string(), // Ensure "Default" deck exists or change this
        model_name: "Basic".to_string(),  // Ensure "Basic" model exists
        fields,
        tags: vec!["programming".to_string(), "rust".to_string()],
    };

    println!("Adding note...");
    let note_id = client.notes().add_note(&note).await?;
    println!("Successfully added note with ID: {}", note_id);

    Ok(())
}
