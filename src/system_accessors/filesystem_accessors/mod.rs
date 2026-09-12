//! Contains accessors for filesystem access.

// ===== Accessors =============================================================

pub(crate) mod file_exists;
#[allow(unused_imports)]
pub(crate) use file_exists::file_exists;

pub(crate) mod read_file_if_exists;
#[allow(unused_imports)]
pub(crate) use read_file_if_exists::read_file_if_exists;

pub(crate) mod read_file;
#[allow(unused_imports)]
pub(crate) use read_file::read_file;

pub(crate) mod write_file;
#[allow(unused_imports)]
pub(crate) use write_file::write_file;

// ===== Helpers ===============================================================

pub(in crate::system_accessors::filesystem_accessors) mod filesystem_errors;

// ===== Test Support ==========================================================

#[cfg(test)]
pub(in crate::system_accessors::filesystem_accessors) mod test_support;
