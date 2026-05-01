#!/usr/bin/env python3
"""
Cross-platform build-and-test for cxx-shared-test-v3.

Uses the cxx-build compile_as_shared_lib API: pure Rust symbol extraction
via the object crate, no bash scripts, no committed artifact files.
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
    repo_root = crate_dir.parent
    lib_dir = repo_root / "target" / "debug"
    bridge_inc = repo_root / "target" / "cxxbridge" / "cxx-shared-test-v3" / "src"
    rust_inc = repo_root / "target" / "cxxbridge" / "rust"
    consumer = crate_dir / "consumer" / "main.cc"

    run(["cargo", "build"], cwd=crate_dir)

    os_name = platform.system()
    if os_name == "Windows":
        run(
            [
                "cl.exe",
                "/EHsc",
                "/std:c++17",
                str(consumer),
                "/DCXX_SHARED",
                f"/I{bridge_inc}",
                f"/I{rust_inc}",
                str(lib_dir / "cxx_shared_test_v3.dll.lib"),
                f"/Fe:{crate_dir / 'test_app.exe'}",
            ],
            cwd=crate_dir,
        )
        env = os.environ | {
            "PATH": str(lib_dir) + os.pathsep + os.environ.get("PATH", "")
        }
        run([str(crate_dir / "test_app.exe")], env=env)
    else:
        run(
            [
                "g++",
                "-std=c++17",
                str(consumer),
                "-DCXX_SHARED",
                f"-I{bridge_inc}",
                f"-I{rust_inc}",
                f"-L{lib_dir}",
                "-lcxx_shared_test_v3",
                f"-Wl,-rpath,{lib_dir}",
                "-o",
                str(crate_dir / "test_app"),
            ],
            cwd=crate_dir,
        )
        run([str(crate_dir / "test_app")])


if __name__ == "__main__":
    main()
