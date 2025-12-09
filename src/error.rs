// Error types for auto-pqdfium-rs

use thiserror::Error;

/// Error types for PDFium operations
#[derive(Error, Debug)]
pub enum PdfiumError {
    #[error("Failed to initialize PDFium library")]
    InitializationFailed,

    #[error("Invalid PDF data")]
    InvalidData,

    #[error("Text extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("PDF to JSON conversion failed: {0}")]
    ConversionFailed(String),
}

/// Convenient Result type for PDFium operations
pub type Result<T> = std::result::Result<T, PdfiumError>;
