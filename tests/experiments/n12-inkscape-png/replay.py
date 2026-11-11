# SPDX-License-Identifier: MIT
"""N-12 one-case feasibility replay; not a production provider or capability API."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True

SOURCE_SHA256 = "87687584b8f4d01360feea9fbf84e510560f44c745a5973e001e55c01a35d1c9"
REQUEST_SHA256 = "14014b129c6790129e28ecd6cf2d26fc591bc25a5a122bfb6bbc0bbeb945c517"

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="new, empty run directory")
    parser.add_argument("--expect", choices=["available", "unavailable"], required=True)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[3]
    os.chdir(repo)
    experiment = Path("tests/experiments/n12-inkscape-png")
    source = Path("skills/storytelling/assets/umbrella-workshop/bench.svg")
    request = experiment / "request.zh-TW.txt"
    source_before = digest(source)
    if source_before != SOURCE_SHA256 or digest(request) != REQUEST_SHA256:
        parser.error("pinned trial input changed; review brief and readback criteria before a new trial")
    renderer = shutil.which("inkscape")
    available = renderer is not None
    if available != (args.expect == "available"):
        parser.error("actual renderer discovery disagrees with expected experiment condition")
    args.output.mkdir(parents=True, exist_ok=False)
    receipt = {"trial": "N-12 / N-R12 G0-G2", "host": "Inkscape",
               "capability": "local SVG to PNG", "platform": platform.system(),
               "request_sha256": digest(request), "source": source.as_posix(),
               "source_sha256_before": source_before,
               "probe": "shutil.which('inkscape')", "available": available,
               "provider_invocations": []}
    if available:
        from readback import readback

        with tempfile.TemporaryDirectory(prefix="n12-inkscape-") as temporary:
            env = dict(os.environ, INKSCAPE_PROFILE_DIR=temporary + "/profile",
                       XDG_CACHE_HOME=temporary + "/cache")
            commands = [("version", [renderer, "--version"]),
                        ("export", [renderer, source.as_posix(), "--export-type=png",
                                    f"--export-filename={args.output / 'bench.png'}",
                                    "--export-width=640", "--export-height=360"])]
            for name, command in commands:
                completed = subprocess.run(command, env=env, capture_output=True, timeout=60)
                (args.output / f"{name}.stdout").write_bytes(completed.stdout)
                (args.output / f"{name}.stderr").write_bytes(completed.stderr)
                (args.output / f"{name}.exit").write_text(str(completed.returncode) + "\n")
                receipt["provider_invocations"].append(command)
                completed.check_returncode()
        png = args.output / "bench.png"
        decoded = readback(png)
        save_json(args.output / "readback.json", decoded)
        receipt.update(status="png_decoded", png_sha256=digest(png),
                       png_bytes=png.stat().st_size, host_version=(args.output / "version.stdout").read_text().strip())
    else:
        shutil.copyfile(experiment / "composition.zh-TW.md", args.output / "composition.zh-TW.md")
        receipt.update(status="brief_delivered_png_pending", host_version="not invoked",
                       missing_capability="SVG-to-PNG export entry point is not exposed on PATH",
                       png_created=False)
    receipt["source_sha256_after"] = digest(source)
    receipt["source_unchanged"] = source_before == receipt["source_sha256_after"]
    if not receipt["source_unchanged"]:
        raise ValueError("source bytes changed")
    save_json(args.output / "receipt.json", receipt)
    print(json.dumps({"status": receipt["status"], "source_unchanged": True}, ensure_ascii=False))


if __name__ == "__main__":
    main()
