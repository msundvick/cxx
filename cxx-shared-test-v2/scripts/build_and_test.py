#!/usr/bin/env python3
"""
Cross-platform build-and-test for cxx-shared-test-v2.

This uses the per-symbol export approach (no CXX_API export macros):
- Linux:   GNU ld version script (glob-based, same as v1)
- macOS:   -Wl,-exported_symbol per symbol (additive, fixes v1's broken -exported_symbols_list)
- Windows: /EXPORT:symbol per symbol (no WHOLEARCHIVE)

The cxx-exports-{os}.txt files list cxx.cc runtime symbols and must exist
before the final cargo build.  This script auto-generates them on first run.
"""

import os
import platform
import subprocess
import sys
from pathlib import Path


def run(cmd, **kwargs):
    print("==>", " ".join(str(a) for a in cmd))
    subprocess.run(cmd, check=True, **kwargs)


def ensure_exports(crate_dir: Path, os_name: str) -> None:
    """Generate cxx-exports-{os}.txt if it doesn't exist yet."""
    exports_dir = crate_dir / "exports"

    if os_name == "Windows":
        exports_file = exports_dir / "cxx-exports-win.txt"
        gen_script = exports_dir / "gen-cxx-exports-win"
    elif os_name == "Darwin":
        exports_file = exports_dir / "cxx-exports-mac.txt"
        gen_script = exports_dir / "gen-cxx-exports-mac"
    else:
        return  # Linux uses version script, no pre-generated file needed

    if exports_file.exists():
        return

    print(f"==> {exports_file.name} not found; generating (requires initial cargo build)...")
    run(["cargo", "build"], cwd=crate_dir)

    if os_name == "Windows":
        bash = r"C:\Program Files\Git\bin\bash.exe"
        run([bash, str(gen_script)], cwd=crate_dir)
    else:
        run(["bash", str(gen_script)], cwd=crate_dir)

    if not exports_file.exists():
        print(f"error: {exports_file} was not created by gen script", file=sys.stderr)
        sys.exit(1)


def main():
    crate_dir = Path(__file__).resolve().parent.parent
    repo_root = crate_dir.parent
    lib_dir = repo_root / "target" / "debug"
    bridge_inc = repo_root / "target" / "cxxbridge" / "cxx-shared-test-v2" / "src"
    rust_inc = repo_root / "target" / "cxxbridge" / "rust"
    consumer = crate_dir / "consumer" / "main.cc"

    os_name = platform.system()

    ensure_exports(crate_dir, os_name)

    run(["cargo", "build"], cwd=crate_dir)

    if os_name == "Windows":
        run(
            [
                "cl.exe",
                "/EHsc",
                "/std:c++17",
                str(consumer),
                "/DCXX_SHARED",           # activates CXX_RUNTIME_API dllimport in cxx.h
                f"/I{bridge_inc}",
                f"/I{rust_inc}",
                str(lib_dir / "cxx_shared_test_v2.dll.lib"),
                f"/Fe:{crate_dir / 'test_app.exe'}",
            ],
            cwd=crate_dir,
        )
        env = os.environ | {
            "PATH": str(lib_dir) + os.pathsep + os.environ.get("PATH", "")
        }
        run([str(crate_dir / "test_app.exe")], env=env)
    else:
        lib_flag = "-lcxx_shared_test_v2"
        run(
            [
                "g++",
                "-std=c++17",
                str(consumer),
                "-DCXX_SHARED",           # activates CXX_RUNTIME_API visibility in cxx.h
                f"-I{bridge_inc}",
                f"-I{rust_inc}",
                f"-L{lib_dir}",
                lib_flag,
                f"-Wl,-rpath,{lib_dir}",
                "-o",
                str(crate_dir / "test_app"),
            ],
            cwd=crate_dir,
        )
        run([str(crate_dir / "test_app")])


if __name__ == "__main__":
    main()
