//! Contains accessors for git access.

// ===== Helpers ===============================================================

pub(in crate::system_accessors::git_accessors) mod git_errors;

// ===== Accessors =============================================================

pub(crate) mod git_add_all;
#[allow(unused_imports)]
pub(crate) use git_add_all::git_add_all;

pub(crate) mod git_commit;
#[allow(unused_imports)]
pub(crate) use git_commit::git_commit;

pub(crate) mod git_init;
#[allow(unused_imports)]
pub(crate) use git_init::git_init;

pub(crate) mod git_signature;
#[allow(unused_imports)]
pub(crate) use git_signature::git_signature;
