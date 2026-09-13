//! Contains template builders for project files.

pub(crate) mod build_gitignore;
#[allow(unused_imports)]
pub(crate) use build_gitignore::build_gitignore;

pub(crate) mod build_license;
#[allow(unused_imports)]
pub(crate) use build_license::build_license;
