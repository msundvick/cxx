use std::path::Path;

use object::read::archive::ArchiveFile;
use object::{Object, ObjectSymbol, SymbolScope};

/// Extract globally-visible, defined symbol names from a static archive whose
/// mangled name contains `pattern`.  Returns a sorted, deduplicated list.
pub(crate) fn extract_symbols(lib_path: &Path, pattern: &str) -> Vec<String> {
    let Ok(data) = std::fs::read(lib_path) else {
        return Vec::new();
    };

    let Ok(archive) = ArchiveFile::parse(data.as_slice()) else {
        return Vec::new();
    };

    let mut symbols = Vec::new();

    for member in archive.members().flatten() {
        let Ok(obj_data) = member.data(data.as_slice()) else {
            continue;
        };
        let Ok(obj) = object::File::parse(obj_data) else {
            continue;
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
