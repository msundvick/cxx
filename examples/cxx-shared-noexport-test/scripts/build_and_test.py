#!/usr/bin/env python3
"""
Cross-platform build-and-test for cxx-shared-noexport-test (legacy staticlib → shared lib workflow).

Demonstrates that CXX_SHARED_LIB is sufficient to export trampolines without
export_macro / CXX_API annotations on user-defined types.

On Windows, CXX_SHARED_LIB causes __declspec(dllexport) on all cxxbridge1
trampolines, replacing the brittle .def-file approach for enumerating mangled
symbol names.

Run from anywhere; all paths are derived from this script's location.
"""
import os
import platform
import subprocess
from pathlib import Path


def run(cmd, **kwargs):
    print("==>", " ".join(str(a) for a in cmd))
    subprocess.run(cmd, check=True, **kwargs)


def main():
    crate_dir = Path(__file__).resolve().parent.parent
    repo_root = crate_dir.parent.parent
    lib_dir = repo_root / "target" / "debug"
    bridge_inc = repo_root / "target" / "cxxbridge" / "cxx-shared-noexport-test" / "src"
    rust_inc = repo_root / "target" / "cxxbridge" / "rust"
    consumer = crate_dir / "consumer" / "main.cc"

    run(["cargo", "build"], cwd=crate_dir)

    os_name = platform.system()
    if os_name == "Windows":
        # Link the Rust staticlib into a DLL.  CXX_SHARED_LIB was defined during
        # the bridge compile step (in build.rs), so trampolines carry
        # __declspec(dllexport) — no .def file needed.
        # /WHOLEARCHIVE pulls in Rust runtime symbols with no direct C++ callers.
        static_lib = lib_dir / "cxx_shared_noexport_test.lib"
        dll = crate_dir / "cxx_shared_noexport_test.dll"
        run([
            "link.exe", "/DLL",
            f"/WHOLEARCHIVE:{static_lib}",
            f"/OUT:{dll}",
        ], cwd=crate_dir)

        import_lib = crate_dir / "cxx_shared_noexport_test.lib"
        run([
            "cl.exe", "/EHsc", "/std:c++17", str(consumer),
            "/DCXX_SHARED",
            f"/I{bridge_inc}", f"/I{rust_inc}",
            str(import_lib),
            f"/Fe:{crate_dir / 'test_app.exe'}",
        ], cwd=crate_dir)
        env = os.environ | {"PATH": str(crate_dir) + os.pathsep + os.environ.get("PATH", "")}
        run([str(crate_dir / "test_app.exe")], env=env)

    elif os_name == "Darwin":
        shared_lib = crate_dir / "libcxx_shared_noexport_test.dylib"
        run([
            "g++", "-dynamiclib",
            "-Wl,-all_load",
            str(lib_dir / "libcxx_shared_noexport_test.a"),
            "-o", str(shared_lib),
        ], cwd=crate_dir)
        run([
            "g++", "-std=c++17", str(consumer),
            "-DCXX_SHARED",
            f"-I{bridge_inc}", f"-I{rust_inc}",
            f"-L{crate_dir}", "-lcxx_shared_noexport_test",
            f"-Wl,-rpath,{crate_dir}",
            "-o", str(crate_dir / "test_app"),
        ], cwd=crate_dir)
        run([str(crate_dir / "test_app")])

    else:  # Linux
        shared_lib = crate_dir / "libcxx_shared_noexport_test.so"
        run([
            "g++", "-shared",
            "-Wl,--whole-archive",
            str(lib_dir / "libcxx_shared_noexport_test.a"),
            "-Wl,--no-whole-archive",
            "-lpthread", "-ldl",
            "-o", str(shared_lib),
        ], cwd=crate_dir)
        run([
            "g++", "-std=c++17", str(consumer),
            "-DCXX_SHARED",
            f"-I{bridge_inc}", f"-I{rust_inc}",
            f"-L{crate_dir}", "-lcxx_shared_noexport_test",
            f"-Wl,-rpath,{crate_dir}",
            "-o", str(crate_dir / "test_app"),
        ], cwd=crate_dir)
        run([str(crate_dir / "test_app")])


if __name__ == "__main__":
    main()
