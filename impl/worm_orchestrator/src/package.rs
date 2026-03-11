/// Imports
use camino::Utf8PathBuf;

/// Represents module info
/// used during compilation phase
pub struct Module {
    /// Canonicalized path to module
    pub path: Utf8PathBuf,

    /// Full module name
    pub name: String,
}

/// Represents package info
/// used during compilation phase
pub struct Package {
    /// Canonicalized path to package
    pub path: Utf8PathBuf,

    /// Package name
    pub name: String,

    /// Non-toposorted package modules
    pub modules: Vec<Module>,
}
