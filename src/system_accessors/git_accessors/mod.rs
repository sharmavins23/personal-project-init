//! Contains accessors for git access.

// ===== Accessors =============================================================

pub(crate) mod git_init;
#[allow(unused_imports)]
pub(crate) use git_init::git_init;

// ===== Helpers ===============================================================

pub(in crate::system_accessors::git_accessors) mod git_errors;

// ===== Test Support ==========================================================

#[cfg(test)]
pub(in crate::system_accessors::git_accessors) mod test_support;
