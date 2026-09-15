//! Template builder for a README's project title block.

// ===== Template Data =========================================================

/// Placeholder locator for the project title.
const PROJECT_TITLE_PLACEHOLDER_MARKER: &str = "{{PROJECT_TITLE}}";

/// String for the README's project title block.
const README_PROJECT_TITLE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/README_TITLE.md"));

// ===== Builder Functions =====================================================

/// Builds the project title block of a generated `README.md`.
///
/// # Parameters
///
/// - [`str`] `project_title`: The project's title.
///
/// # Returns
///
/// - [`String`]: The title block, including its trailing blank line.
///
#[allow(dead_code)]
#[must_use]
pub(crate) fn build_readme_project_title(project_title: &str) -> String {
    // Substitute the project title.
    README_PROJECT_TITLE_TEMPLATE.replace(PROJECT_TITLE_PLACEHOLDER_MARKER, project_title)
}

// ===== Unit Testing ==========================================================

/// Unit tests for README project title building.
#[cfg(test)]
mod test_build_readme_project_title {

    use crate::{
        template_formatters::{
            build_readme_project_title,
            build_readme_project_title::PROJECT_TITLE_PLACEHOLDER_MARKER,
        },
        unit_testing::test_file::SAFE_FILE_CONTENT_CHARS,
    };
    use proptest::{prop_assert, proptest};

    // ----- Test Cases --------------------------------------------------------

    proptest! {

        /// Project title must land in the output.
        #[test]
        fn substitutes_any_project_title(project_title in SAFE_FILE_CONTENT_CHARS) {
            // Act.
            let result: String = build_readme_project_title(&project_title);

            // Assert.
            prop_assert!(result.contains(&project_title));
            prop_assert!(!result.contains(PROJECT_TITLE_PLACEHOLDER_MARKER));
        }

    }
}
