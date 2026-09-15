//! Filesystem accessor function for reading a file.

use crate::system_accessors::filesystem_accessors::filesystem_errors::file_read_error_context;
use color_eyre::eyre::{Context, Result};
use std::{fs::read_to_string, path::Path};

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
pub(crate) fn read_file(path: &Path) -> Result<String> {
    read_to_string(path).wrap_err(file_read_error_context(path))
}

// ===== Unit Testing ==========================================================

/// Unit tests for file reading.
#[cfg(test)]
mod test_read_file {

    use crate::{
        system_accessors::filesystem_accessors::read_file,
        unit_testing::{
            test_directory::{SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory},
            test_file::{SAFE_FILE_CONTENT_CHARS, SAFE_FILENAME_CHARS, TestFile, create_test_file},
        },
    };
    use color_eyre::eyre::Result;
    use proptest::{prop_assert_eq, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Reading a file must return its contents.
        #[test]
        fn existing_file_returns_content(directory_name in SAFE_DIRECTORY_CHARS, file_content in SAFE_FILE_CONTENT_CHARS, file_name in SAFE_FILENAME_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);
            let test_file: TestFile = create_test_file(Some(&file_content), &file_name, &test_directory);

            // Act.
            let result: Result<String> = read_file(&test_file.file_path);

            // Assert.
            let returned_content: String = result.expect("File read should succeed on non-empty file.");
            prop_assert_eq!(returned_content, file_content);
        }

    }
}
