use std::path::Path;

use object::read::archive::ArchiveFile;
use object::{Object, ObjectSymbol, SymbolScope};

/// Extract globally-visible, defined symbol names from a static archive whose
/// mangled name contains `pattern`.  Returns a sorted, deduplicated list.
pub(crate) fn extract_symbols(lib_path: &Path, pattern: &str) -> Vec<String> {
    let data = match std::fs::read(lib_path) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    let archive = match ArchiveFile::parse(data.as_slice()) {
        Ok(a) => a,
        Err(_) => return Vec::new(),
    };

    let mut symbols = Vec::new();

    for member in archive.members().flatten() {
        let obj_data = match member.data(data.as_slice()) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let obj = match object::File::parse(obj_data) {
            Ok(o) => o,
            Err(_) => continue,
        };
        for sym in obj.symbols() {
            if sym.is_undefined() {
                continue;
            }
            // Keep symbols visible at dynamic link time or to the static
            // linkage unit (covers both ELF/Mach-O global and COFF public).
            if !matches!(sym.scope(), SymbolScope::Dynamic | SymbolScope::Linkage) {
                continue;
            }
            if let Ok(name) = sym.name() {
                if name.contains(pattern) {
                    symbols.push(name.to_owned());
                }
            }
        }
    }

    symbols.sort_unstable();
    symbols.dedup();
    symbols
}
