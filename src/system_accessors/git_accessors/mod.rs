//! Contains accessors for git access.

// ===== Accessors =============================================================

pub(crate) mod git_add_all;
#[allow(unused_imports)]
pub(crate) use git_add_all::git_add_all;

pub(crate) mod git_init;
#[allow(unused_imports)]
pub(crate) use git_init::git_init;

// ===== Helpers ===============================================================

pub(in crate::system_accessors::git_accessors) mod git_errors;
