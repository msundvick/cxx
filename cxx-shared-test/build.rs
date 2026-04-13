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

    if target_os == "windows" && target_env == "msvc" {
        // MSVC dead-code stripping would otherwise remove the generated C++
        // wrappers because Rust never calls them directly.  /WHOLEARCHIVE
        // forces the linker to keep every object file in the archive.
        // Use the full OUT_DIR path so the linker finds the archive reliably.
        let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
        let lib_path = PathBuf::from(&out_dir).join(format!("{}.lib", crate_name));
        println!("cargo:rustc-link-arg-cdylib=/WHOLEARCHIVE:{}", lib_path.display());
    } else if target_os == "macos" {
        // Apple ld: -all_load only applies to archives that follow it in the
        // command line, but cargo appends rustc-link-arg-cdylib flags after its
        // own -l flags, so -all_load arrives too late.  Use -force_load with
        // the explicit archive path instead — it is position-independent.
        let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
        let lib_path = PathBuf::from(&out_dir).join(format!("lib{}.a", crate_name));
        println!("cargo:rustc-link-arg-cdylib=-Wl,-force_load,{}", lib_path.display());
    } else {
        // GNU/LLVM ld: Cargo's default version script hides everything with
        // `local: *`.  We supply our own script that promotes the bridge
        // symbols to `global` before that rule fires.
        //
        // The wildcard is derived from the crate name so this template works
        // without modification for any crate.
        let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
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

    build.compile(&env::var("CARGO_PKG_NAME").unwrap().replace('-', "_"));
}
