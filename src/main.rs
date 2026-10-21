//! Contains the main entrypoint for the binary.

use color_eyre::{Result, install};
use ppi::{ExecutionContext, parse_argument_environment_context};

// ===== Driver Code ===========================================================

/// Main entrypoint for the binary.
fn main() -> Result<()> {
    // Configure `color-eyre` for the entire project.
    install()?;

    // Parse all arguments into an execution-wide argument context.
    let Some(execution_context): Option<ExecutionContext> = parse_argument_environment_context()?
    else {
        // The `--help` or `--version` commands were ran; Do nothing else.
        return Ok(());
    };

    println!("{execution_context:#?}");

    // Initialize the git repository.

    // Read user credentials.

    // Build & write the README.md file.

    // Build & write the LICENSE.md file.

    // Build & write the .gitignore file.

    // Stage & commit all changes.

    Ok(())
}
