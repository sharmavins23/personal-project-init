//! Git accessor function for reading configured credentials.

use crate::system_accessors::git_accessors::git_errors::git_signature_error_context;
use color_eyre::eyre::{Context, Result};
use git2::{Repository, Signature};

// ===== Accessor Code =========================================================

/// Reads the git credentials configured for the repository at `path`.
///
/// # Parameters
///
/// - [`Path`] `path`: The repository to read configuration from.
///
/// # Returns
///
/// A [`Result`] which is:
///
/// - [`Signature`]: The configured `user.name` and `user.email`.
/// - [`Err`]: Returns the identity lookup error.
///
#[allow(dead_code)]
pub(crate) fn git_signature(repository: &Repository) -> Result<Signature<'static>> {
    repository
        .signature()
        .wrap_err(git_signature_error_context(repository))
}

// ===== Unit Testing ==========================================================

/// Unit tests for git credential lookup.
#[cfg(test)]
mod test_git_signature {

    use crate::{
        system_accessors::git_accessors::git_signature::git_signature,
        unit_testing::{
            test_directory::{SAFE_DIRECTORY_CHARS, TestDirectory, create_test_directory},
            test_git::{SAFE_GIT_EMAIL_CHARS, SAFE_GIT_NAME_CHARS, configure_test_credentials},
        },
    };
    use color_eyre::eyre::Result;
    use git2::{Repository, Signature};
    use proptest::{prop_assert_eq, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Configured credentials must be returned verbatim.
        #[test]
        fn configured_credentials_are_returned(
            directory_name in SAFE_DIRECTORY_CHARS,
            user_name in SAFE_GIT_NAME_CHARS,
            user_email in SAFE_GIT_EMAIL_CHARS,
        ) {
            // Arrange.
            let test_directory: TestDirectory = create_test_directory(&directory_name);
            let repository: Repository = Repository::init(&test_directory.directory_path)
                .expect("Git init should succeed as expected.");
            configure_test_credentials(&test_directory, &user_name, &user_email);

            // Act.
            let result: Result<Signature<'static>> =
                git_signature(&repository);

            // Assert.
            let signature: Signature<'static> =
                result.expect("Signature lookup should succeed as expected.");
            prop_assert_eq!(
                signature.name().expect("Name should be valid UTF-8."),
                user_name.as_str()
            );
            prop_assert_eq!(
                signature.email().expect("Email should be valid UTF-8."),
                user_email.as_str()
            );
        }

    }
}
