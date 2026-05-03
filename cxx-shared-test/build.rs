fn main() {
    cxx_build::CFG.export_macro = Some("CXX_API");

    let mut build = cxx_build::bridge("src/lib.rs");
    build.std("c++14");
    build.define("CXX_EXPORTING", "1");

    // Emit the cdylib linker directive to prevent symbol stripping on Linux/macOS
    println!("cargo:rustc-link-arg-cdylib=-Wl,--export-dynamic-symbol=cxxbridge1*");
    println!("cargo:rustc-link-arg-cdylib=-Wl,--export-dynamic-symbol=cxx_shared_test$*");
    // cxx-ai/build.rs (around line 11)

    println!("cargo:rustc-link-arg-cdylib=/WHOLEARCHIVE:cxx-shared-test.lib");

    build.compile("cxx-shared-test");
}
