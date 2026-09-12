//! Contains accessors for filesystem access.

pub(in crate::system_accessors::filesystem_accessors) mod filesystem_errors;

pub(crate) mod read_file;
#[allow(unused_imports)]
pub(crate) use read_file::run;

#[cfg(test)]
pub(in crate::system_accessors::filesystem_accessors) mod test_support;
