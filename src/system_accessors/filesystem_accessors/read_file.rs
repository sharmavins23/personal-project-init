//! Filesystem accessor function for reading a file.

use crate::system_accessors::filesystem_accessors::filesystem_errors::file_read_error_context;
use color_eyre::eyre::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

// ===== Accessor Code =========================================================

/// Reads the entire file at `path` into a string.
///
/// # Parameters
///
/// - [`Path`] `path`: The file path to read from.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`String`]: The contents of the file.
/// - [`Err`]: Returns the file read error.
///
#[allow(dead_code)]
pub(crate) fn run(path: &Path) -> Result<String> {
    read_to_string(path).wrap_err(file_read_error_context(path))
}

// ===== Unit Testing ==========================================================

/// Unit tests for file reading.
#[cfg(test)]
mod test_read_file {

    use super::run;
    use crate::system_accessors::filesystem_accessors::test_support::{TestFile, create_test_file};
    use color_eyre::eyre::Result;
    use proptest::{prop_assert_eq, proptest};

    // ----- Test Cases --------------------------------------------------------

    /// A missing file must yield an error report.
    #[test]
    fn missing_file_yields_error() {
        // Arrange.
        let test_file: TestFile = create_test_file("missing.txt", None);

        // Act.
        let result: Result<String> = run(&test_file.file_path);

        // Assert.
        assert!(result.is_err());
    }

    proptest! {
        /// Any items written to files must read back, verbatim.
        #[test]
        fn read_returns_written_content_verbatim(file_content in ".*") {
            // Arrange.
            let test_file: TestFile = create_test_file("sample.txt", Some(file_content.as_str()));

            // Act.
            let result: Result<String> = run(&test_file.file_path);

            // Assert.
            let returned_content: String = result.expect("File read should succeed on non-empty file.");
            prop_assert_eq!(returned_content, file_content);
        }
    }
}
