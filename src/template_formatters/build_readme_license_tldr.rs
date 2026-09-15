//! Template builder for a README's license TL;DR block.

// ===== Template Data =========================================================

/// Sentinel used for checking whether a License TL;DR already exists.
#[allow(dead_code)]
pub(crate) const LICENSE_TLDR_SENTINEL_MARKER: &str = "# License TL;DR";

/// String for the README's license TL;DR block.
const README_LICENSE_TLDR_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/README_LICENSE_TLDR.md"
));

// ===== Builder Functions =====================================================

/// Builds the license TL;DR block of a generated `README.md`.
///
/// # Returns
///
/// - [`str`]: The TL;DR block.
///
#[allow(dead_code)]
#[must_use]
pub(crate) const fn build_readme_license_tldr() -> &'static str {
    README_LICENSE_TLDR_TEMPLATE
}

