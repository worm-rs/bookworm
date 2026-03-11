/// Imports
use camino::Utf8PathBuf;

/// Represents compilation session
pub struct CompileSess {
    /// Compilation outcome path
    pub outcome: Utf8PathBuf,

    /// Types context
    pub ctxt: TyCtxt,
}
