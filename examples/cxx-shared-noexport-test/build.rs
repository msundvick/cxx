fn main() {
    // Legacy workflow: export_macro is NOT set, so no CXX_API annotations are
    // emitted on user-defined types.  The caller is responsible for managing
    // symbol visibility at the DLL boundary.
    //
    // CXX_SHARED_LIB marks the cxx runtime trampolines (cxxbridge1$...) as
    // visibility("default") / __declspec(dllexport).  This is required so they
    // survive into the final shared library whether the C++ bridge is compiled
    // by Cargo (as here) or externally by CMake compiling lib.rs.cc.
    //
    // On Windows this eliminates the need for a .def file: the trampolines are
    // exported automatically via __declspec(dllexport) rather than requiring
    // each mangled symbol name to be listed manually.
    cxx_build::bridge("src/lib.rs")
        .std("c++14")
        .compile("cxx_shared_noexport_test");
}
