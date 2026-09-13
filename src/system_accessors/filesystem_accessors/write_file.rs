//! Filesystem accessor function for writing a file.

use crate::system_accessors::filesystem_accessors::filesystem_errors::file_write_error_context;
use color_eyre::eyre::{Context, Result};
use std::fs::write;
use std::path::Path;

// ===== Accessor Code =========================================================

/// Writes `file_content` to the file at `path`, replacing it if it exists.
///
/// # Parameters
///
/// - [`Path`] `path`: The file path to write to.
/// - [`str`] `file_content`: The contents to write into the file.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`()`]: The file was written successfully.
/// - [`Err`]: Returns the file write error.
///
#[allow(dead_code)]
pub(crate) fn write_file(path: &Path, file_content: &str) -> Result<()> {
    write(path, file_content).wrap_err(file_write_error_context(path))
}

// ===== Unit Testing ==========================================================

/// Unit tests for file writing.
#[cfg(test)]
mod test_write_file {

    use super::write_file;
    use crate::system_accessors::filesystem_accessors::test_support::{
        SAFE_FILENAME_CHARS, TestFile, create_test_file,
    };
    use proptest::{prop_assert_eq, proptest};
    use std::fs::read_to_string;

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Any items written to files must read back, verbatim.
        #[test]
        fn written_file_succeeds(file_content in ".*", file_name in SAFE_FILENAME_CHARS) {
            // Arrange.
            let test_file: TestFile = create_test_file(&file_name, None);

            // Act.
            write_file(&test_file.file_path, file_content.as_str())
                .expect("File write should succeed as expected.");

            // Assert.
            let returned_content: String = read_to_string(&test_file.file_path)
                .expect("File read should succeed as expected.");

            prop_assert_eq!(returned_content, file_content);
        }

    }
}
