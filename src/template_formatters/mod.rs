//! Contains template builders for project files.

pub(crate) mod build_gitignore;
#[allow(unused_imports)]
pub(crate) use build_gitignore::build_gitignore;

pub(crate) mod build_license;
#[allow(unused_imports)]
pub(crate) use build_license::build_license;

pub(crate) mod build_readme_license_tldr;
#[allow(unused_imports)]
pub(crate) use build_readme_license_tldr::build_readme_license_tldr;

pub(crate) mod build_readme_project_title;
#[allow(unused_imports)]
pub(crate) use build_readme_project_title::build_readme_project_title;
