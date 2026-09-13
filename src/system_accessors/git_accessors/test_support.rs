//! Supports for git accessor unit testing.

use std::path::PathBuf;
use tempfile::{TempDir, tempdir};

/// A directory-safe character class, for proptest generation.
pub(in crate::system_accessors::git_accessors) const SAFE_DIRECTORY_CHARS: &str =
    "[a-zA-Z0-9][a-zA-Z0-9_.-]{0,31}";

// ===== Type Definitions ======================================================

/// Stores a (temporary) directory path and directory, used for testing.
pub(in crate::system_accessors::git_accessors) struct TestDirectory {
    /// The full path to the test directory.
    pub(in crate::system_accessors::git_accessors) directory_path: PathBuf,
    /// Test directory.
    #[allow(dead_code)]
    pub(in crate::system_accessors::git_accessors) test_directory: TempDir,
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
pub(in crate::system_accessors::git_accessors) fn create_test_directory(
    directory_name: &str,
) -> TestDirectory {
    // Create the root test directory.
    let test_directory: TempDir = tempdir().expect("Failed to create test directory.");

    // Create the nested directory path.
    let directory_path: PathBuf = test_directory.path().join(directory_name);
    std::fs::create_dir_all(&directory_path).expect("Failed to create nested test directory.");

    TestDirectory {
        directory_path,
        test_directory,
    }
}
