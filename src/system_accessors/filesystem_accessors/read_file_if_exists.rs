//! Filesystem accessor function for reading a file, if it exists.

use crate::system_accessors::filesystem_accessors::filesystem_errors::file_read_error_context;
use color_eyre::eyre::{Context, Result};
use std::fs::read_to_string;
use std::io::ErrorKind::NotFound;
use std::path::Path;

// ===== Accessor Code =========================================================

/// Reads the entire file at `path` into a string, if it exists.
///
/// As opposed to the
/// [`crate::system_accessors::filesystem_accessors::read_file`] function, this
/// function atomically returns context on whether a file exists or not.
///
/// # Parameters
///
/// - [`Path`] `path`: The file path to read from.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`Some`]: The contents of the file, if it exists.
/// - [`None`]: The file does not exist.
/// - [`Err`]: Returns the file read error.
///
#[allow(dead_code)]
pub(crate) fn read_file_if_exists(path: &Path) -> Result<Option<String>> {
    // Check if `read_to_string()` returns content.
    match read_to_string(path) {
        // Return content.
        Ok(file_content) => Ok(Some(file_content)),
        // Determine the kind of error.
        Err(error) => {
            if error.kind() == NotFound {
                // The file was not found; No content to return.
                Ok(None)
            } else {
                // Another issue occurred.
                Err(error).wrap_err(file_read_error_context(path))
            }
        }
    }
}

// ===== Unit Testing ==========================================================

/// Unit tests for conditional file reading.
#[cfg(test)]
mod test_read_file_if_exists {

    use super::read_file_if_exists;
    use crate::system_accessors::filesystem_accessors::test_support::{
        SAFE_FILENAME_CHARS, TestFile, create_test_file,
    };
    use color_eyre::eyre::Result;
    use proptest::{prop_assert, prop_assert_eq, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Reading a missing file must return nothing, not an error.
        #[test]
        fn missing_file_yields_none(file_name in SAFE_FILENAME_CHARS) {
            // Arrange.
            let test_file: TestFile = create_test_file(&file_name, None);

            // Act.
            let result: Result<Option<String>> = read_file_if_exists(&test_file.file_path);

            // Assert.
            assert!(
                result
                    .expect("Read should succeed for a missing file.")
                    .is_none()
            );
        }

        /// Fuzz test containing various files and names.
        #[test]
        fn test_read_file_if_exists(file_content in ".*", file_name in SAFE_FILENAME_CHARS) {
            // Arrange.
            let test_file: TestFile = create_test_file(&file_name, Some(file_content.as_str()));

            // Act.
            let result: Result<Option<String>> = read_file_if_exists(&test_file.file_path);

            // Assert.
            let returned_content: Option<String> =
                result.expect("File read should succeed on an existing file.");
            prop_assert!(returned_content.is_some());
            prop_assert_eq!(returned_content.expect("Contents should be present."), file_content);
        }

    }
}
