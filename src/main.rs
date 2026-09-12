//! Contains the main entrypoint for the binary.

use color_eyre::{Result, install};

// ===== Driver Code ===========================================================

/// Main entrypoint for the binary.
fn main() -> Result<()> {
    // Configure `color-eyre` for the entire project.
    install()?;

    println!("Hello from PPI!");

    Ok(())
}
