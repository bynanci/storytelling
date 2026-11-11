"""Pack and install the portable npm skill in a clean project, entirely offline."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
SKILL = Path("skills/storytelling")
PACKAGE_NAME = "@bynanci/storytelling"
REQUIRED_SKILL_FILES = {
    "SKILL.md", "LICENSE", "agents/openai.yaml",
    "assets/umbrella-workshop/bench.svg", "assets/umbrella-workshop/taps.wav",
    "references/multimodal-templates.md", "references/workshop-demo.md",
    "references/examples.md", "references/graph-loops.md",
    "references/graphify-tools.md", "references/video-cli.md",
    "scripts/check_trace.py", "scripts/graph_plan.py",
}


def file_names(directory):
    return {path.relative_to(directory).as_posix()
            for path in directory.rglob("*") if path.is_file()}


class DistributionTest(unittest.TestCase):
    def test_pack_and_install(self):
        npm = shutil.which("npm.cmd" if os.name == "nt" else "npm")
        self.assertIsNotNone(npm, "npm is required to test the distribution")
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary).resolve()

            def run_npm(directory, *args):
                result = subprocess.run(
                    [npm, *args, "--offline", "--ignore-scripts", "--no-audit",
                     "--no-fund", "--cache", str(work / "npm-cache")],
                    cwd=directory, capture_output=True, text=True, timeout=60,
                )
                self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
                return result.stdout

            packed, = json.loads(run_npm(ROOT, "pack", "--json", "--pack-destination", str(work)))
            self.assertEqual(packed["name"], PACKAGE_NAME)
            source = ROOT / SKILL
            expected = file_names(source)
            self.assertFalse(REQUIRED_SKILL_FILES - expected, REQUIRED_SKILL_FILES - expected)
            package_paths = {item["path"] for item in packed["files"]}
            metadata = {"package.json", "README.md", "README.zh-TW.md", "README.en.md", "LICENSE"}
            self.assertEqual(package_paths, metadata | {f"{SKILL.as_posix()}/{p}" for p in expected})

            consumer = work / "consumer"
            consumer.mkdir()
            (consumer / "package.json").write_text('{"name":"consumer","private":true}', encoding="utf-8")
            run_npm(consumer, "install", "--save-dev", str(work / packed["filename"]))
            installed = consumer / "node_modules" / PACKAGE_NAME
            manifest = json.loads((installed / "package.json").read_text(encoding="utf-8"))
            self.assertEqual(manifest["name"], PACKAGE_NAME)
            for key in ("dependencies", "optionalDependencies", "peerDependencies",
                        "bundleDependencies", "bundledDependencies"):
                self.assertFalse(manifest.get(key), f"Unexpected runtime dependencies in {key}")
            hooks = {"preinstall", "install", "postinstall", "prepublish", "preprepare",
                     "prepare", "postprepare", "prepack", "postpack"}
            self.assertFalse(hooks.intersection(manifest.get("scripts", {})), "Unexpected lifecycle hook")
            self.assertEqual(file_names(installed), package_paths)
            for relative in sorted(expected):
                with self.subTest(relative=relative):
                    self.assertEqual((installed / SKILL / relative).read_bytes(),
                                     (source / relative).read_bytes())


if __name__ == "__main__":
    unittest.main()
