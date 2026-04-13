#!/usr/bin/env python3
"""
Cross-platform build-and-test for cxx-static-test (staticlib linked directly into exe).
Run from anywhere; all paths are derived from this script's location.
"""
import platform
import subprocess
from pathlib import Path


def run(cmd, **kwargs):
    print("==>", " ".join(str(a) for a in cmd))
    subprocess.run(cmd, check=True, **kwargs)


def main():
    crate_dir = Path(__file__).resolve().parent.parent
    repo_root = crate_dir.parent
    lib_dir = repo_root / "target" / "debug"
    bridge_inc = repo_root / "target" / "cxxbridge" / "cxx-static-test" / "src"
    rust_inc = repo_root / "target" / "cxxbridge" / "rust"
    consumer = crate_dir / "consumer" / "main.cc"

    run(["cargo", "build"], cwd=crate_dir)

    os_name = platform.system()
    if os_name == "Windows":
        # MSVC links the static lib directly into the exe.
        run([
            "cl.exe", "/EHsc", "/std:c++17", "/MD", str(consumer),
            f"/I{bridge_inc}", f"/I{rust_inc}",
            str(lib_dir / "cxx_static_test.lib"),
            f"/Fe:{crate_dir / 'test_app.exe'}",
        ], cwd=crate_dir)
        run([str(crate_dir / "test_app.exe")])
    else:
        run([
            "g++", "-std=c++17", str(consumer),
            f"-I{bridge_inc}", f"-I{rust_inc}",
            f"-L{lib_dir}", "-lcxx_static_test",
            "-lpthread", "-ldl",
            "-o", str(crate_dir / "test_app"),
        ], cwd=crate_dir)
        run([str(crate_dir / "test_app")])


if __name__ == "__main__":
    main()
