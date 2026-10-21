//! Model for the input passed via CLI into PPI.

use clap::Parser;
use std::path::PathBuf;

// ===== Definitions ===========================================================

/// Initializes a personal project repository at the given location
#[derive(Debug, Parser)]
pub struct ArgumentInput {
    /// Directory to initialize (default: current directory)
    #[arg(value_name = "DIRECTORY")]
    pub directory: Option<PathBuf>,

    /// Demonstrate what would be written without changing anything
    #[arg(long, short)]
    pub dry_run: bool,
}
