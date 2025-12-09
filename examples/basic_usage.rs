use auto_pqdfium_rs::{extract_text, pdf_to_json, cleanup};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get PDF path from command line or use test sample
    let pdf_path = env::args().nth(1).unwrap_or_else(|| {
        "tests/sample.pdf".to_string()
    });

    println!("Processing PDF: {}\n", pdf_path);

    // Read PDF file
    let pdf_bytes = fs::read(&pdf_path)?;
    println!("PDF size: {} bytes\n", pdf_bytes.len());

    // Extract text
    println!("=== TEXT EXTRACTION ===");
    match extract_text(&pdf_bytes) {
        Ok(text) => {
            println!("Extracted text ({} characters):", text.len());
            println!("{}\n", text);
        }
        Err(e) => {
            eprintln!("Text extraction failed: {}\n", e);
        }
    }

    // Convert to JSON
    println!("=== PDF TO JSON ===");
    match pdf_to_json(&pdf_bytes) {
        Ok(json) => {
            println!("JSON output ({} characters):", json.len());

            // Pretty print first 500 characters
            if json.len() > 500 {
                println!("{}...\n", &json[..500]);
            } else {
                println!("{}\n", json);
            }

            // Parse and show structure
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
                if let Some(obj) = parsed.as_object() {
                    println!("JSON has {} top-level keys:", obj.len());
                    for key in obj.keys() {
                        println!("  - {}", key);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("JSON conversion failed: {}", e);
        }
    }

    // Cleanup (optional)
    cleanup();
    println!("\nDone!");

    Ok(())
}
