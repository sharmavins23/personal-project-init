//! Test fixture for creating (temporary) test files.

use crate::unit_testing::test_directory::TestDirectory;
use std::{fs::write, path::PathBuf};

// ===== Fixture Constants =====================================================

/// A filename-safe character class, for proptest generation.
#[cfg(test)]
pub(crate) const SAFE_FILENAME_CHARS: &str = "[a-zA-Z0-9]([a-zA-Z0-9_.-]{0,30}[a-zA-Z0-9])?";

/// A file content-safe character class, for proptest generation.
#[cfg(test)]
pub(crate) const SAFE_FILE_CONTENT_CHARS: &str = ".*";

// ===== Fixture Models ========================================================

/// Stores a (temporary) file path and directory, used for testing.
#[cfg(test)]
pub(crate) struct TestFile {
    /// The full path to the test file.
    pub(crate) file_path: PathBuf,
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
pub(crate) fn create_test_file(
    file_content: Option<&str>,
    file_name: &str,
    test_directory: &TestDirectory,
) -> TestFile {
    // Create the file within the nested directory path.
    let file_path: PathBuf = test_directory.directory_path.join(file_name);

    // If `file_content` is specified, write to the file.
    if let Some(provided_file_content) = file_content {
        write(&file_path, provided_file_content)
            .expect("Failed to seed test file with provided content.");
    }

    TestFile { file_path }
}
