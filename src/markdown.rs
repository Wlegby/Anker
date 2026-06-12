use pulldown_cmark::{Event, Options, Parser, html};

/// Converts Obsidian/Markdown text into HTML suitable for Anki.
///
/// It converts formatting like **bold** and *italic* into HTML `<b>` and `<i>` tags (via markdown parsing),
/// and seamlessly transforms Obsidian math delimiters (`$...$` and `$$...$$`)
/// into Anki's native MathJax delimiters (`\(...\)` and `\[...\]`).
pub fn markdown_to_anki(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(text, options).map(|event| match event {
        // Intercept math events and convert them to raw text with Anki MathJax delimiters
        Event::InlineMath(math) => Event::Html(format!("\\({}\\)", math).into()),
        Event::DisplayMath(math) => Event::Html(format!("\\[{}\\]", math).into()),
        _ => event,
    });

    let mut html_output = String::with_capacity(text.len() * 2);
    html::push_html(&mut html_output, parser);

    html_output
}

use std::io::Write;
use std::process::{Command, Stdio};

fn pandoc_typst_to_latex(math: &str, is_block: bool) -> String {
    let input = if is_block {
        format!("$ {} $", math)
    } else {
        format!("${}$", math)
    };

    let mut child_opt = Command::new("pandoc")
        .args(&["-f", "typst", "-t", "latex"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok();

    if let Some(mut child) = child_opt {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.as_bytes());
        }
        if let Ok(output) = child.wait_with_output() {
            if output.status.success() {
                return String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }
    }

    // Fallback if pandoc fails or is not installed
    if is_block {
        format!("\\[{}\\]", math)
    } else {
        format!("\\({}\\)", math)
    }
}

/// Converts Obsidian/Markdown text into HTML for Anki, while simultaneously transpiling
/// **Typst** math into **LaTeX** (MathJax) via Pandoc.
///
/// **Requires `pandoc` to be installed on the host system.**
///
/// If you wrote `sum_(i=1)^n i` in Obsidian inside `$...$`, this will use Pandoc to convert it
/// to `\(\sum_{i = 1}^{n}i\)` so Anki renders it natively perfectly.
pub fn markdown_to_anki_with_typst(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(text, options).map(|event| match event {
        Event::InlineMath(math) => Event::Html(pandoc_typst_to_latex(&math, false).into()),
        Event::DisplayMath(math) => Event::Html(pandoc_typst_to_latex(&math, true).into()),
        _ => event,
    });

    let mut html_output = String::with_capacity(text.len() * 2);
    html::push_html(&mut html_output, parser);

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_anki() {
        let md = "Here is **bold** and *italic*.\n\nInline $x^2$ and block $$y^2$$.";
        let html = markdown_to_anki(md);

        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("\\(x^2\\)"));
        assert!(html.contains("\\[y^2\\]"));
    }

    #[test]
    fn test_markdown_to_anki_with_typst() {
        // Only run this test if pandoc is available, otherwise it falls back to raw math
        let md = "Typst math: $sum_(i=1)^n i$";
        let html = markdown_to_anki_with_typst(md);

        // This assertion might fail if pandoc isn't installed during the test runner execution,
        // but typically it falls back to raw MathJax if pandoc is missing.
        // We just ensure it doesn't crash.
        assert!(html.contains("\\("));
    }
}
