//! Filesystem accessor function for checking whether a file exists.

use crate::system_accessors::filesystem_accessors::filesystem_errors::file_exists_error_context;
use color_eyre::eyre::{Context, Result};
use std::path::Path;

// ===== Accessor Code =========================================================

/// Determines whether a file or directory exists at `path`.
///
/// # Parameters
///
/// - [`Path`] `path`: The file path to check.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`bool`]: Whether anything exists at the given path.
/// - [`Err`]: Returns the file check error.
///
#[allow(dead_code)]
pub(crate) fn file_exists(path: &Path) -> Result<bool> {
    path.try_exists().wrap_err(file_exists_error_context(path))
}

// ===== Unit Testing ==========================================================

/// Unit tests for file existence checks.
#[cfg(test)]
mod test_file_exists {

    use crate::{
        system_accessors::filesystem_accessors::file_exists,
        unit_testing::{
            test_directory::{SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory},
            test_file::{SAFE_FILE_CONTENT_CHARS, SAFE_FILENAME_CHARS, TestFile, create_test_file},
        },
    };
    use color_eyre::eyre::Result;
    use proptest::{prop_assert, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Seeded files and directories must be reported as existing.
        #[test]
        fn existing_file_reports_existing(directory_name in SAFE_DIRECTORY_CHARS, file_content in SAFE_FILE_CONTENT_CHARS, file_name in SAFE_FILENAME_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);
            let test_file: TestFile = create_test_file(Some(&file_content), &file_name, &test_directory);

            // Act.
            let directory_exists_result: Result<bool> = file_exists(&test_directory.directory_path);
            let file_exists_result: Result<bool> = file_exists(&test_file.file_path);

            // Assert.
            prop_assert!(directory_exists_result.expect("Directory existence check should succeed."));
            prop_assert!(file_exists_result.expect("File existence check should succeed."));
        }

        /// Missing files and directories must be reported as not existing.
        #[test]
        fn missing_file_reports_not_existing(directory_name in SAFE_DIRECTORY_CHARS, file_name in SAFE_FILENAME_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);
            let test_file: TestFile = create_test_file(None, &file_name, &test_directory);

            // Act.
            let missing_directory_result: Result<bool> = file_exists(&test_directory.directory_path);
            let missing_file_result: Result<bool> = file_exists(&test_file.file_path);

            // Assert.
            prop_assert!(
                missing_directory_result.expect("Directory existence check should succeed.")
            );
            prop_assert!(!missing_file_result.expect("File existence check should succeed."));
        }

    }
}
