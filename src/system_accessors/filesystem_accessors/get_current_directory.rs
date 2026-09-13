//! Filesystem accessor function for retrieving the current (working) directory.

use crate::system_accessors::filesystem_accessors::filesystem_errors::get_current_directory_error_context;
use color_eyre::eyre::{Context, Result};
use std::{env::current_dir, path::PathBuf};

// ===== Accessor Code =========================================================

/// Determines the directory that PPI was invoked from.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`PathBuf`]: The path to the working directory.
/// - [`Err`]: Returns the working directory lookup error.
///
#[allow(dead_code)]
pub(crate) fn get_current_directory() -> Result<PathBuf> {
    current_dir().wrap_err(get_current_directory_error_context())
}

// ===== Unit Testing ==========================================================

/// Unit tests for current directory lookup.
#[cfg(test)]
mod test_get_current_directory {

    use super::get_current_directory;
    use crate::unit_testing::test_directory::{
        SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory,
    };
    use color_eyre::eyre::Result;
    use proptest::{prop_assert_eq, proptest};
    use std::{env::set_current_dir, fs::canonicalize, path::PathBuf};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// The process's current directory must be reported back.
        #[test]
        fn reports_the_process_directory(directory_name in SAFE_DIRECTORY_CHARS) {
            // Get the current directory to restore after testing.
            let initial_directory: PathBuf = get_current_directory().expect("Failed to read the initial directory.");

            // * Create a new directory and verify its location.

            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);

            // Act.
            set_current_dir(&test_directory.directory_path).expect("Failed to move to test directory.");
            let result: Result<PathBuf> = get_current_directory();
            set_current_dir(&initial_directory).expect("Failed to restore initial directory.");

            // Assert.
            let result: PathBuf = result.expect("Working directory lookup should succeed.");
            prop_assert_eq!(
                canonicalize(result).expect("Failed to canonicalize reported directory."),
                canonicalize(&test_directory.directory_path).expect("Failed to canonicalize expected path.")
            );
        }

    }
}
