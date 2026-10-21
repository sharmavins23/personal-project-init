//! Orchestrator for argument parsing and environment context resolution.

use crate::{
    models::{ArgumentInput, ExecutionContext},
    system_accessors::filesystem_accessors::get_current_directory,
};
use clap::{
    Parser,
    error::ErrorKind::{DisplayHelp, DisplayVersion},
};
use color_eyre::eyre::{Context, Result, eyre};
use heck::ToTitleCase;
use std::path::{Path, PathBuf};

// ===== Helper Functions ======================================================

/// Parses the command-line arguments into an [`ArgumentInput`].
///
/// `--help` or `--version` requests are treated as expected outcomes rather
/// than Clap's default errors (which otherwise immediately abort the program).
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`Some`]: The parsed argument input.
/// - [`None`]: Clap printed the help or version output.
/// - [`Err`]: Returns the argument parsing or output error.
///
fn parse_argument_inputs() -> Result<Option<ArgumentInput>> {
    match ArgumentInput::try_parse() {
        // Argument inputs were successfully parsed.
        Ok(argument_input) => Ok(Some(argument_input)),
        // Argument inputs were not parsed, but this is a help/version call.
        Err(error) if matches!(error.kind(), DisplayHelp | DisplayVersion) => {
            error.print()?;
            Ok(None)
        }
        // Parsing failed.
        Err(error) => Err(error.into()),
    }
}

/// Derives the project title from the directory's final path component.
///
/// # Parameters
///
/// - [`Path`] `directory`: The directory whose name becomes the title.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`String`]: The title-cased directory name.
/// - [`Err`]: Returns the name lookup or conversion error.
///
fn parse_directory_name(directory: &Path) -> Result<String> {
    // Parse the raw directory name.
    let directory_name: &str = directory
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            eyre!(
                "Failed to read a UTF-8 directory name from `{}`.",
                directory.display()
            )
        })?;

    // Derive a project title.
    let title: String = directory_name.to_title_case();

    if title.is_empty() {
        return Err(eyre!(
            "Failed to derive a project title from `{}`.",
            directory.display()
        ));
    }

    Ok(title)
}

/// Resolves the directory that PPI should operate on.
///
/// If `directory_input` is provided, it is canonicalized so relative paths and
/// symlinks resolve to an absolute path. Otherwise, the current working
/// directory is returned.
///
/// # Parameters
///
/// - [`Option<&PathBuf>`] `directory_input`: The directory passed on the
///   command line, if any.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`PathBuf`]: The resolved directory.
/// - [`Err`]: Returns the canonicalization or working directory lookup error.
///
fn retrieve_specified_directory(directory_input: Option<&PathBuf>) -> Result<PathBuf> {
    directory_input.map_or_else(get_current_directory, |directory| {
        directory
            .canonicalize()
            .wrap_err_with(|| format!("Failed to resolve directory `{}`.", directory.display()))
    })
}

// ===== Driver Code ===========================================================

/// Parses the command-line arguments and resolves the execution context.
///
/// # Returns
///
/// - [`Some`]: The resolved execution context.
/// - [`None`]: Clap printed the help or version output.
///
/// # Errors
///
/// Returns an error if the arguments cannot be parsed or the execution context
/// cannot be resolved.
///
pub fn parse_argument_environment_context() -> Result<Option<ExecutionContext>> {
    // Parse input arguments.
    let Some(argument_input): Option<ArgumentInput> = parse_argument_inputs()? else {
        // No need to keep processing.
        return Ok(None);
    };

    // Get the directory referenced.
    let directory: PathBuf = retrieve_specified_directory(argument_input.directory.as_ref())?;

    // Get the title.
    let title: String = parse_directory_name(&directory)?;

    // Form our execution context.
    Ok(Some(ExecutionContext {
        argument_input,
        directory,
        title,
    }))
}

// ===== Unit Testing ==========================================================

/// Unit tests for argument and environment parsing.
#[cfg(test)]
mod test_parse_argument_environment_context {

    use super::{parse_directory_name, retrieve_specified_directory};
    use crate::unit_testing::test_directory::{
        SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory,
    };
    use color_eyre::eyre::Result;
    use heck::ToTitleCase;
    use proptest::{prop_assert_eq, proptest};
    use std::{fs::canonicalize, path::PathBuf};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// A specified directory must be canonicalized.
        #[test]
        fn canonicalizes_specified_directory(directory_name in SAFE_DIRECTORY_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);

            // Act.
            let result: Result<PathBuf> =
                retrieve_specified_directory(Some(&test_directory.directory_path));

            // Assert.
            let directory: PathBuf = result.expect("Directory resolution should succeed.");
            prop_assert_eq!(
                directory,
                canonicalize(&test_directory.directory_path)
                    .expect("Test directory should canonicalize.")
            );
        }

        /// The directory's name must become its title-cased form.
        #[test]
        fn derives_title_from_directory_name(directory_name in SAFE_DIRECTORY_CHARS) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);

            // Act.
            let result: Result<String> = parse_directory_name(&test_directory.directory_path);

            // Assert.
            let title: String = result.expect("Title parsing should succeed.");
            prop_assert_eq!(title, directory_name.to_title_case());
        }

    }
}
