//! Template builder for a project's `LICENSE.md` file.

use chrono::{Datelike, Local};

// ===== Template Data =========================================================

/// String for `LICENSE.md` contents.
const LICENSE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/LICENSE.md"));

/// Placeholder locator for the git user's name.
const GIT_USER_PLACEHOLDER_MARKER: &str = "{{GIT_USER}}";

/// Placeholder locator for the current year.
const YEAR_PLACEHOLDER_MARKER: &str = "{{YEAR}}";

// ===== Builder Functions =====================================================

#[allow(dead_code)]
#[must_use]
pub(crate) fn build_license(git_user: &str) -> String {
    // Determine the current year for the copyright notice.
    let current_year: i32 = Local::now().year();

    // Substitute placeholders with their values.
    LICENSE_TEMPLATE
        .replace(GIT_USER_PLACEHOLDER_MARKER, git_user)
        .replace(YEAR_PLACEHOLDER_MARKER, &current_year.to_string())
}

// ===== Unit Testing ==========================================================

/// Unit tests for license building.
#[cfg(test)]
mod test_build_license {

    use crate::{
        template_formatters::{
            build_license,
            build_license::{GIT_USER_PLACEHOLDER_MARKER, YEAR_PLACEHOLDER_MARKER},
        },
        unit_testing::test_file::SAFE_FILE_CONTENT_CHARS,
    };
    use chrono::{Datelike, Local};
    use proptest::{prop_assert, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Git user and current year must both land in the output.
        #[test]
        fn substitutes_user_and_year(git_user in SAFE_FILE_CONTENT_CHARS) {
            // Arrange.
            let current_year: String = Local::now().year().to_string();

            // Act.
            let result: String = build_license(&git_user);

            // Assert.
            prop_assert!(result.contains(&current_year));
            prop_assert!(!result.contains(YEAR_PLACEHOLDER_MARKER));
            prop_assert!(result.contains(&git_user));
            prop_assert!(!result.contains(GIT_USER_PLACEHOLDER_MARKER));

        }

    }
}
