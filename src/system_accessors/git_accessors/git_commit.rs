//! Git accessor function for committing staged changes.

use crate::system_accessors::git_accessors::git_errors::git_commit_error_context;
use color_eyre::eyre::{Context, Result};
use git2::{
    Commit, Diff, ErrorCode::UnbornBranch, Index, Oid, Reference, Repository, Signature, Tree,
};

// ===== Helper Functions ======================================================

/// Returns the commit that `HEAD` currently points to, or nothing if the
/// repository is freshly initialized.
///
///
/// # Parameters
///
/// - [`Repository`] `repository`: The repository to read `HEAD` from.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`Some`]: The commit that `HEAD` points to.
/// - [`None`]: `HEAD` is unborn, so the repository has no commits.
/// - [`Err`]: Returns the `HEAD` lookup or commit peel error.
///
fn get_latest_commit(repository: &Repository) -> Result<Option<Commit<'_>>> {
    match repository
        .head()
        .and_then(|head: Reference<'_>| head.peel_to_commit())
    {
        // We successfully found a commit.
        Ok(parent) => Ok(Some(parent)),
        // There are no commits; The branch has yet to have a first commit.
        Err(error) if error.code() == UnbornBranch => Ok(None),
        // We failed to find any commits.
        Err(error) => Err(error).wrap_err(git_commit_error_context(repository)),
    }
}

/// Returns the tree of `parent_commit`, if a parent commit exists.
///
/// # Parameters
///
/// - [`Option<Commit>`] `parent_commit`: The commit whose tree to read. `None`
///   when `HEAD` is unborn.
/// - [`Repository`] `repository`: The repository used to build error context.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`Some`]: The parent commit's tree.
/// - [`None`]: There is no parent commit.
/// - [`Err`]: Returns the tree lookup error.
///
fn get_parent_commit_tree<'repo>(
    parent_commit: Option<&Commit<'repo>>,
    repository: &'repo Repository,
) -> Result<Option<Tree<'repo>>> {
    match parent_commit {
        Some(parent) => Ok(Some(
            parent
                .tree()
                .wrap_err(git_commit_error_context(repository))?,
        )),
        None => Ok(None),
    }
}

// ===== Accessor Code =========================================================

/// Commits the staged index in `repository` and updates `HEAD`.
///
/// # Parameters
///
/// - [`str`] `message`: The commit message.
/// - [`Repository`] `repository`: The repository to commit in.
/// - [`Signature`] `signature`: The git credentials for the author and
///   committer.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`()`]: The commit was created, or there was nothing staged to commit.
/// - [`Err`]: Returns the git commit error.
///
#[allow(dead_code)]
pub(crate) fn git_commit(
    message: &str,
    repository: &Repository,
    signature: &Signature<'static>,
) -> Result<()> {
    // Retrieve the latest commit and its tree.
    let parent_commit: Option<Commit<'_>> = get_latest_commit(repository)?;
    let parent_tree: Option<Tree<'_>> = get_parent_commit_tree(parent_commit.as_ref(), repository)?;

    // Get the repository's index file.
    let mut index: Index = repository
        .index()
        .wrap_err(git_commit_error_context(repository))?;

    // Short-cut: If no change occurs, we can skip our commit.
    let file_difference: Diff<'_> = repository
        .diff_tree_to_index(parent_tree.as_ref(), Some(&index), None)
        .wrap_err(git_commit_error_context(repository))?;
    if file_difference.deltas().next().is_none() {
        return Ok(());
    }

    // Write the new staged changes as a tree.
    let new_tree_oid: Oid = index
        .write_tree()
        .wrap_err(git_commit_error_context(repository))?;
    let new_tree: Tree<'_> = repository
        .find_tree(new_tree_oid)
        .wrap_err(git_commit_error_context(repository))?;

    // Compile all commit actors.
    let update_ref: Option<&str> = Some("HEAD");
    let author: &Signature<'_> = signature;
    let committer: &Signature<'_> = signature;
    let parents: Vec<&Commit<'_>> = parent_commit.iter().collect();
    let tree: &Tree<'_> = &new_tree;

    // Perform the full commit.
    repository
        .commit(update_ref, author, committer, message, tree, &parents)
        .wrap_err(git_commit_error_context(repository))?;

    Ok(())
}

// ===== Unit Testing ==========================================================

/// Unit tests for committing staged changes.
#[cfg(test)]
mod test_git_commit {

    use crate::{
        system_accessors::git_accessors::git_commit::git_commit,
        unit_testing::{
            test_directory::{SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory},
            test_file::{SAFE_FILE_CONTENT_CHARS, SAFE_FILENAME_CHARS, TestFile, create_test_file},
            test_git::{SAFE_GIT_EMAIL_CHARS, SAFE_GIT_NAME_CHARS},
        },
    };
    use color_eyre::eyre::Result;
    use git2::{IndexAddOption, Repository, Signature};
    use proptest::{prop_assert, prop_assert_eq, proptest};
    use std::path::Path;

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// A staged file must land in an initial commit.
        #[test]
        fn staged_file_is_committed(
            directory_name in SAFE_DIRECTORY_CHARS,
            file_content in SAFE_FILE_CONTENT_CHARS,
            file_name in SAFE_FILENAME_CHARS,
            user_name in SAFE_GIT_NAME_CHARS,
            user_email in SAFE_GIT_EMAIL_CHARS,
        ) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);
            let message: &str = "chore: Initial commit";
            let _test_file: TestFile =
                create_test_file(Some(&file_content), &file_name, &test_directory);

            let repository: Repository = Repository::init(&test_directory.directory_path)
                .expect("Git init should succeed as expected.");

            let mut index = repository.index().expect("Index should open.");
            index
                .add_all(["*"], IndexAddOption::DEFAULT, None)
                .expect("Staging should succeed as expected.");
            index.write().expect("Index should write as expected.");

            let signature: Signature<'static> = Signature::now(&user_name, &user_email)
                .expect("Signature should be created as expected.");

            // Act.
            let result: Result<()> = git_commit(message, &repository, &signature);

            // Assert.
            prop_assert!(result.is_ok());
            let head = repository
                .head()
                .expect("HEAD should exist after committing.")
                .peel_to_commit()
                .expect("HEAD should peel to a commit.");
            prop_assert_eq!(
                head.message().expect("Message should be valid UTF-8."),
                message
            );
            prop_assert_eq!(head.parent_count(), 0);
            prop_assert!(
                head.tree()
                    .expect("Tree should exist.")
                    .get_path(Path::new(&file_name))
                    .is_ok()
            );

            let author = head.author();
            prop_assert_eq!(
                author.name().expect("Author name should be valid UTF-8."),
                user_name.as_str()
            );
            prop_assert_eq!(
                author.email().expect("Author email should be valid UTF-8."),
                user_email.as_str()
            );
        }
    }
}
