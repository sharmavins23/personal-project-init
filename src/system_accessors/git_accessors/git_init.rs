//! Git accessor function for initializing a repository.

use crate::system_accessors::git_accessors::git_errors::git_init_error_context;
use color_eyre::eyre::{Context, Result};
use gix::{Repository, init, open};
use std::path::Path;

// ===== Accessor Code =========================================================

/// Initializes a git repository in the directory at `path`, if a repository
/// does not already exist.
///
/// # Parameters
///
/// - [`Path`] `path`: The directory to initialize a repository in.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`Repository`]: The repository was initialized successfully.
/// - [`Err`]: Returns the git initialization error.
///
#[allow(dead_code)]
pub(crate) fn git_init(path: &Path) -> Result<Repository> {
    if let Ok(repository) = open(path) {
        return Ok(repository);
    }

    init(path).wrap_err(git_init_error_context(path))
}

// ===== Unit Testing ==========================================================

/// Unit tests for git initialization.
#[cfg(test)]
mod test_git_init {

    use super::git_init;
    use crate::unit_testing::test_directory::{
        SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory,
    };
    use color_eyre::eyre::Result;
    use gix::Repository;
    use proptest::{prop_assert, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Initialization always lands `.git` at exactly the requested path.
        #[test]
        fn init_succeeds(directory_name in SAFE_DIRECTORY_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);

            // Act.
            let result: Result<Repository> = git_init(&test_directory.directory_path);

            // Assert.
            prop_assert!(result.is_ok());
            prop_assert!(test_directory.directory_path.join(".git").is_dir());
            prop_assert!(!test_directory.test_directory.path().join(".git").exists());
        }

        /// Re-initialization should not fail.
        #[test]
        fn idempotent_init_succeeds(directory_name in SAFE_DIRECTORY_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);

            // Act.
            let first_initialization_result: Result<Repository> = git_init(&test_directory.directory_path);
            let second_initialization_result: Result<Repository> = git_init(&test_directory.directory_path);

            // Assert.
            prop_assert!(first_initialization_result.is_ok());
            prop_assert!(second_initialization_result.is_ok());
            prop_assert!(test_directory.directory_path.join(".git").is_dir());
            prop_assert!(!test_directory.test_directory.path().join(".git").exists());
        }

    }
}
