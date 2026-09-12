//! Entrypoint for the binary.

use color_eyre::{Result, install};

// ===== Driver Code ===========================================================

/// Entrypoint for the binary.
fn main() -> Result<()> {
    install()?;

    println!("Hello from PPI!");

    Ok(())
}
