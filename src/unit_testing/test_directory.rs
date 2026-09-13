//! Test fixture for creating (temporary) test directories.

use std::{fs::create_dir, path::PathBuf};
use tempfile::{TempDir, tempdir};

// ===== Fixture Constants =====================================================

/// A directory-safe character class, for proptest generation.
#[cfg(test)]
pub(crate) const SAFE_DIRECTORY_CHARS: &str = "[a-zA-Z0-9]{1,32}";

// ===== Fixture Models ========================================================

/// Stores a (temporary) directory path and directory, used for testing.
#[cfg(test)]
pub(crate) struct TestDirectory {
    /// The full path to the test directory.
    pub(crate) directory_path: PathBuf,
    /// Test directory.
    #[allow(dead_code)]
    pub(crate) test_directory: TempDir,
}

// ===== Fixture Functions =====================================================

/// Creates a (temporary) test directory with a nested directory named
/// `directory_name`.
///
/// # Parameters
///
/// - [`str`] `directory_name`: The nested directory to create. Pass `"."` to
///   use the root temporary directory itself.
///
/// # Returns
///
/// A [`TestDirectory`] object holding the directory and its path.
///
#[cfg(test)]
pub(crate) fn create_test_directory(directory_name: &str) -> TestDirectory {
    // Create the root test directory.
    let test_directory: TempDir = tempdir().expect("Failed to create test directory.");

    // Create the nested directory path.
    let directory_path: PathBuf = test_directory.path().join(directory_name);
    create_dir(&directory_path).expect("Failed to create nested test directory.");

    TestDirectory {
        directory_path,
        test_directory,
    }
}
