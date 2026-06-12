use std::process::Command;

/// Launches the Anki application on the host machine.
///
/// This simply runs the `anki` command in the background. It assumes `anki` is in your PATH.
pub fn launch_anki() -> std::io::Result<()> {
    Command::new("anki")
        // We spawn it detached so our Rust program isn't blocked and doesn't wait for Anki to exit.
        .spawn()?;

    Ok(())
}
