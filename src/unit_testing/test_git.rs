//! Test fixture for configuring git credentials.

use crate::unit_testing::test_directory::TestDirectory;
use git2::Repository;

// ===== Fixture Constants =====================================================

/// A git name-safe character class, for proptest generation.
#[cfg(test)]
pub(crate) const SAFE_GIT_NAME_CHARS: &str =
    "[A-Za-z][A-Za-z0-9._-]{0,15}( [A-Za-z0-9._-]{1,15}){0,2}";

/// A git email-safe character class, for proptest generation.
#[cfg(test)]
pub(crate) const SAFE_GIT_EMAIL_CHARS: &str =
    "[a-z][a-z0-9._-]{0,31}@[a-z][a-z0-9.-]{0,15}\\.[a-z]{2,8}";

// ===== Fixture Functions =====================================================

/// Writes local git credentials into the repository at `test_directory`.
///
/// # Parameters
///
/// - [`TestDirectory`] `test_directory`: The initialized repository.
/// - [`str`] `user_name`: The git user name to configure.
/// - [`str`] `user_email`: The git user email to configure.
///
#[cfg(test)]
pub(crate) fn configure_test_credentials(
    test_directory: &TestDirectory,
    user_name: &str,
    user_email: &str,
) {
    let repository: Repository = Repository::open(&test_directory.directory_path)
        .expect("Repository should open before configuring credentials.");
    let mut config = repository
        .config()
        .expect("Repository config should open before configuring credentials.");

    config
        .set_str("user.name", user_name)
        .expect("Git user name should be set.");
    config
        .set_str("user.email", user_email)
        .expect("Git user email should be set.");
}
