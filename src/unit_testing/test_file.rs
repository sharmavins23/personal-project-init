//! Test fixture for creating (temporary) test files.

use std::path::PathBuf;
use tempfile::{TempDir, tempdir};

// ===== Fixture Constants =====================================================

/// A filename-safe character class, for proptest generation.
#[cfg(test)]
pub(crate) const SAFE_FILENAME_CHARS: &str = "[a-zA-Z0-9][a-zA-Z0-9_.-]{0,31}";

// ===== Fixture Models ========================================================

/// Stores a (temporary) file path and directory, used for testing.
#[cfg(test)]
pub(crate) struct TestFile {
    /// The full path to the test file.
    pub(crate) file_path: PathBuf,
    /// Test directory.
    #[allow(dead_code)]
    pub(crate) test_directory: TempDir,
}

// ===== Fixture Functions =====================================================

/// Creates a (temporary) test directory and file.
///
/// # Parameters
///
/// - [`Option<&str>`] `file_content`: Contents to populate the file with,
///   if provided. If empty, the file will not be created.
/// - [`str`] `file_name`: The name of the file to create.
///
/// # Returns
///
/// A [`TestFile`] object holding the directory and file path.
///
#[cfg(test)]
pub(crate) fn create_test_file(file_content: Option<&str>, file_name: &str) -> TestFile {
    // Create the test directory.
    let test_directory: TempDir = tempdir().expect("Failed to create test directory.");

    // Create the included filepath.
    let file_path: PathBuf = test_directory.path().join(file_name);

    // If `file_content` is specified, write to the file.
    if let Some(provided_file_content) = file_content {
        std::fs::write(&file_path, provided_file_content)
            .expect("Failed to seed test file with provided content.");
    }

    TestFile {
        file_path,
        test_directory,
    }
}
