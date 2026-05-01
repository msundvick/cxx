use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");

    // No export_macro: symbols are exported via per-symbol linker args instead.
    // cargo_metadata(false) suppresses the default link directive so we can
    // re-emit it with +whole-archive to force C++ bridge objects into the output.
    let bridges = ["src/lib.rs"];
    cxx_build::bridges(bridges)
        .cargo_metadata(false)
        .std("c++14")
        .compile(&crate_name);

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static:+whole-archive={}", crate_name);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_os == "windows" && target_env == "msvc" {
        let emit_export = |symbol: &str| {
            println!("cargo::rustc-link-arg=/EXPORT:{}", symbol);
        };

        // cxx.cc (cxxbridge1) symbols - pre-extracted, committed to source
        let exports_file = manifest_dir.join("exports/cxx-exports-win.txt");
        println!("cargo::rerun-if-changed={}", exports_file.display());
        if exports_file.exists() {
            for symbol in read_symbol_file(&exports_file) {
                emit_export(&symbol);
            }
        }

        // bridge lib symbols - extracted at build time
        let bridge_lib = Path::new(&out_dir).join(format!("{}.lib", crate_name));
        let script = manifest_dir.join("exports/extract-symbols-win");
        for symbol in extract_symbols(&script, "cxx_shared_test_v2", &bridge_lib) {
            emit_export(&symbol);
        }
    } else if target_os == "macos" {
        // -installed_name makes the dylib relocatable via @rpath
        let lib_name = format!("lib{}.dylib", crate_name);
        println!(
            "cargo::rustc-link-arg=-Wl,-install_name,@rpath/{}",
            lib_name
        );

        let emit_export = |symbol: &str| {
            // -exported_symbol is additive; -exported_symbols_list overrides rustc's list
            println!("cargo::rustc-link-arg=-Wl,-exported_symbol,{}", symbol);
        };

        // cxx.cc (cxxbridge1) symbols - pre-extracted, committed to source
        let exports_file = manifest_dir.join("exports/cxx-exports-mac.txt");
        println!("cargo::rerun-if-changed={}", exports_file.display());
        if exports_file.exists() {
            for symbol in read_symbol_file(&exports_file) {
                emit_export(&symbol);
            }
        }

        // bridge lib symbols - extracted at build time
        let bridge_lib = Path::new(&out_dir).join(format!("lib{}.a", crate_name));
        let script = manifest_dir.join("exports/extract-symbols-mac");
        for symbol in extract_symbols(&script, "cxx_shared_test_v2", &bridge_lib) {
            emit_export(&symbol);
        }
    } else {
        // GNU ld version script - glob approach works well on Linux
        let map_path = Path::new(&out_dir).join("export.map");
        std::fs::write(
            &map_path,
            format!(
                "{{\n  global:\n    *{crate}*;\n    *rust10cxxbridge1*;\n    cxxbridge1*;\n  local:\n    *;\n}};\n",
                crate = crate_name
            ),
        )
        .unwrap();
        println!(
            "cargo::rustc-link-arg=-Wl,--version-script={}",
            map_path.display()
        );
    }
}

fn read_symbol_file(path: &Path) -> Vec<String> {
    let contents = std::fs::read_to_string(path).unwrap_or_default();
    contents
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect()
}

fn extract_symbols(script: &Path, pattern: &str, lib_path: &Path) -> Vec<String> {
    if !lib_path.exists() {
        return Vec::new();
    }

    let mut cmd = if cfg!(windows) {
        let mut c = Command::new(r"C:\Program Files\Git\bin\bash.exe");
        c.arg(script);
        c
    } else {
        Command::new(script)
    };

    let output = cmd
        .arg(pattern)
        .arg(lib_path)
        .output()
        .unwrap_or_else(|_| panic!("failed to run {}", script.display()));

    if !output.status.success() {
        eprintln!(
            "extract-symbols script exited {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}
