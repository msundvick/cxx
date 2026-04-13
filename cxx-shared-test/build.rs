use std::{env, fs, path::PathBuf};

fn main() {
    // Inject the export macro name into codegen so every generated struct,
    // opaque type, and free function is decorated with CXX_API.
    cxx_build::CFG.export_macro = Some("CXX_API");

    let mut build = cxx_build::bridge("src/lib.rs");
    build.std("c++14");
    build.define("CXX_SHARED_LIB", "1");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    let out_dir = env::var("OUT_DIR").unwrap();
    let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");

    build.compile(&crate_name);

    if target_os == "windows" && target_env == "msvc" {
        // MSVC dead-code stripping would otherwise remove the generated C++
        // wrappers because Rust never calls them directly.  /WHOLEARCHIVE
        // forces the linker to keep every object file in the archive.
        // Use the full OUT_DIR path so the linker finds the archive reliably.
        let lib_path = PathBuf::from(&out_dir).join(format!("{}.lib", crate_name));
        println!("cargo:rustc-link-arg-cdylib=/WHOLEARCHIVE:{}", lib_path.display());
    } else if target_os == "macos" {
        // Two things are needed to export C++ bridge symbols from a cdylib:
        //
        // 1. -force_load: Apple ld only extracts archive members that satisfy
        //    a referenced symbol.  Rust never calls the C++ wrappers directly,
        //    so without this flag none of the bridge objects are included.
        //    Unlike -all_load (which is position-sensitive), -force_load names
        //    the archive explicitly and works regardless of link-arg ordering.
        //
        // 2. -exported_symbols_list: rustc passes its own list to ld64 that
        //    restricts exports to Rust-known symbols (#[no_mangle] etc.).
        //    C++ bridge symbols are not in that list, so even though -force_load
        //    includes the objects, the symbols are suppressed from the export
        //    table.  ld64 treats multiple -exported_symbols_list args as
        //    additive (unioned), so a supplemental file adds to rustc's list
        //    rather than replacing it.
        let lib_path = PathBuf::from(&out_dir).join(format!("lib{}.a", crate_name));
        let exports_path = PathBuf::from(&out_dir).join("exports.txt");
        fs::write(
            &exports_path,
            format!("*{crate}*\n*cxxbridge1*\n", crate = crate_name),
        )
        .unwrap();
        println!("cargo:rustc-link-arg-cdylib=-Wl,-force_load,{}", lib_path.display());
        println!("cargo:rustc-link-arg-cdylib=-Wl,-exported_symbols_list,{}", exports_path.display());
    } else {
        // GNU/LLVM ld: Cargo's default version script hides everything with
        // `local: *`.  We supply our own script that promotes the bridge
        // symbols to `global` before that rule fires.
        //
        // The wildcard is derived from the crate name so this template works
        // without modification for any crate.
        let map_path = PathBuf::from(&out_dir).join("export.map");

        fs::write(
            &map_path,
            format!(
                "{{\n  global:\n    *{crate}*;\n    *rust*cxxbridge1*;\n    cxxbridge1*;\n  local:\n    *;\n}};\n",
                crate = crate_name
            ),
        )
        .unwrap();

        println!(
            "cargo:rustc-link-arg-cdylib=-Wl,--version-script={}",
            map_path.display()
        );

        // Force the linker to retain the generated C++ wrapper object files
        // even though Rust never calls them directly.  We scope --whole-archive
        // tightly to avoid pulling in duplicate cxxbridge1 symbols.
        println!("cargo:rustc-link-arg-cdylib=-Wl,--whole-archive");
        println!("cargo:rustc-link-arg-cdylib=-l{}", crate_name);
        println!("cargo:rustc-link-arg-cdylib=-Wl,--no-whole-archive");
    }
}
