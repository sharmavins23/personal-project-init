//! Git accessor function for staging all changes.

use crate::system_accessors::git_accessors::git_errors::git_add_all_error_context;
use color_eyre::eyre::{Context, Result};
use git2::{Index, IndexAddOption, Repository};
use std::path::Path;

// ===== Accessor Code =========================================================

/// Stages all changes in the repository at `path`, as `git add --all` would.
///
/// Ignored files are skipped, matching `git add`.
///
/// # Parameters
///
/// - [`Path`] `path`: The repository to stage changes in.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`()`]: The changes were staged successfully.
/// - [`Err`]: Returns the git staging error.
///
#[allow(dead_code)]
pub(crate) fn git_add_all(path: &Path) -> Result<()> {
    // Open the repository and its index.
    let repository: Repository =
        Repository::open(path).wrap_err(git_add_all_error_context(path))?;
    let mut index: Index = repository
        .index()
        .wrap_err(git_add_all_error_context(path))?;

    // Stage every matching path.
    index
        .add_all(["*"], IndexAddOption::DEFAULT, None)
        .wrap_err(git_add_all_error_context(path))?;

    // Write the index back to `.git/index`.
    index.write().wrap_err(git_add_all_error_context(path))
}

// ===== Unit Testing ==========================================================

/// Unit tests for staging all changes.
#[cfg(test)]
mod test_git_add_all {

    use crate::{
        system_accessors::git_accessors::git_add_all::git_add_all,
        unit_testing::{
            test_directory::{SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory},
            test_file::{SAFE_FILE_CONTENT_CHARS, SAFE_FILENAME_CHARS, TestFile, create_test_file},
        },
    };
    use color_eyre::eyre::Result;
    use git2::{Blob, IndexEntry, Repository};
    use proptest::{prop_assert, prop_assert_eq, proptest};
    use std::path::Path;

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// A written file must appear in the index after staging.
        #[test]
        fn staged_file_enters_index(
            directory_name in SAFE_DIRECTORY_CHARS,
            file_content in SAFE_FILE_CONTENT_CHARS,
            file_name in SAFE_FILENAME_CHARS,
        ) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);
            let _test_file: TestFile = create_test_file(Some(&file_content), &file_name, &test_directory);
            Repository::init(&test_directory.directory_path).expect("Git init should succeed as expected.");

            // Act.
            let result: Result<()> = git_add_all(&test_directory.directory_path);

            // Assert.
            prop_assert!(result.is_ok());
            let repository: Repository =
                Repository::open(&test_directory.directory_path).expect("Repo should open.");
            let index = repository.index().expect("Index should open.");
            let entry: IndexEntry = index
                .get_path(Path::new(&file_name), 0)
                .expect("Staged file should be present in the index.");
            let blob: Blob = repository
                .find_blob(entry.id)
                .expect("Staged blob should be readable.");
            prop_assert_eq!(blob.content(), file_content.as_bytes());
        }

    }
}
