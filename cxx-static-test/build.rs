fn main() {
    // Intentionally do NOT set CFG.export_macro — the static-lib workflow
    // never needs dllexport/visibility annotations on the generated headers.
    cxx_build::bridge("src/lib.rs")
        .std("c++14")
        .compile("cxx_static_test");
}
