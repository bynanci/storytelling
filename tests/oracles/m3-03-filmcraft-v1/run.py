"""Build a fresh synthetic review and exercise journaled FilmCraft readback."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import textwrap

HOST_SHA256 = "064a1f55cde5f6e1ac04bcd3a5b72e5d7366905f3d7333793e78dda0c109d33f"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def invoke(argv, *, cwd, env=None):
    result = subprocess.run(argv, cwd=cwd, env=env, capture_output=True)
    if result.returncode:
        raise RuntimeError(
            f"command failed ({result.returncode}): {argv[0]}\n"
            + result.stderr.decode(errors="replace")
        )
    return result.stdout


def dump(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--storytelling", type=Path, required=True)
    parser.add_argument("--filmcraft", type=Path, required=True)
    parser.add_argument("--ffmpeg", type=Path, default=Path("/usr/bin/ffmpeg"))
    parser.add_argument("--python", type=Path, default=Path(sys.executable))
    parser.add_argument("--cargo", type=Path, required=True)
    parser.add_argument("--run-root", type=Path, required=True)
    args = parser.parse_args()
    for path in (args.storytelling, args.filmcraft, args.ffmpeg, args.python, args.cargo):
        if not path.is_absolute() or not path.is_file():
            raise ValueError(f"explicit absolute executable missing: {path}")
    root = args.run_root
    if not root.is_absolute() or root.exists():
        raise ValueError("run root must be new and absolute; existing paths are never reused")
    if digest(args.filmcraft.read_bytes()) != HOST_SHA256:
        raise ValueError("FilmCraft binary SHA-256 is not the reviewed v0.18.0 executable")
    if invoke([str(args.filmcraft), "--version"], cwd=Path.cwd()).decode().strip() != "filmcraft-cli 0.18.0":
        raise ValueError("unexpected FilmCraft version")

    repo = Path(__file__).resolve().parents[3]
    root.mkdir(parents=True)
    (root / "logs").mkdir()
    source = root / "source.mp4"
    invoke(
        [
            str(args.ffmpeg), "-v", "error", "-f", "lavfi", "-i",
            "color=0xff0000:s=1920x1080:r=25:d=2", "-an", "-c:v", "libx264",
            "-pix_fmt", "yuv420p", str(source),
        ],
        cwd=root,
    )
    source_sha = digest(source.read_bytes())
    receipt = json.loads(
        invoke(
            [str(args.storytelling), "verify-source", str(source), source_sha], cwd=root
        )
    )
    if receipt.get("verification", {}).get("sha256") != source_sha:
        raise ValueError("source-byte verification did not match generated media")

    request = {
        "schema_version": 1,
        "brief": {
            "schema_version": 1,
            "brief_id": "m3-03-proposal-a",
            "intent": "one selected synthetic picture cut",
            "assets": [{"id": "source-video"}],
            "manual_selected_asset_ids": ["source-video"],
            "hard_constraints": [],
            "preferences": {"preferred_asset_ids": []},
        },
        "candidates": [{
            "asset_id": "source-video",
            "content_hash": source_sha,
            "duration": {"numerator": "2", "denominator": "1"},
            "source_range": {
                "start": {"numerator": "1", "denominator": "25"},
                "end": {"numerator": "26", "denominator": "25"},
            },
            "score": 10,
        }],
        "max_duration": {"numerator": "1", "denominator": "1"},
        "max_shots": 1,
    }
    request_path = root / "planner-request.json"
    dump(request_path, request)
    proposal = invoke([str(args.storytelling), "plan-story", str(request_path)], cwd=root)
    (root / "proposal.json").write_bytes(proposal)

    fixture = json.loads(
        (repo / "tests/fixtures/openfilm/finite-v1/video-trim.json").read_text()
    )
    fixture["composition"].update(id="m3-03-review", storyId="synthetic-m3-03", duration=1)
    fixture["composition"]["tracks"][0]["clips"] = [{
        "id": "source-video-occurrence",
        "assetId": "source-video",
        "sourceIn": 0.04,
        "sourceOut": 1.04,
        "timelineStart": 0,
        "timelineDuration": 1,
    }]
    fixture["assets"][0].update(
        id="source-video",
        uri=source.as_uri(),
        name="synthetic source video",
        duration=2,
        contentHash=source_sha,
    )
    fixture["settings"]["frameRate"] = 25
    snapshot = root / "snapshot.json"
    dump(snapshot, fixture)
    context = json.loads(
        (repo / "tests/fixtures/story-review-v1/context.json").read_text()
    )
    context.update(project_id="synthetic-m3-03-project", target_revision="new-isolated-project")
    context["host"].update(
        version="0.18.0",
        profile_id="filmcraft-25-picture-only-v1",
        binary_sha256=HOST_SHA256,
    )
    context_path = root / "context.json"
    dump(context_path, context)
    bundle = invoke(
        [
            str(args.storytelling), "prepare-story-review", str(root / "proposal.json"),
            str(snapshot), str(context_path),
        ],
        cwd=root,
    )
    (root / "review-bundle.json").write_bytes(bundle)
    otio = invoke(
        [
            str(args.storytelling), "export-story-otio", str(root / "proposal.json"),
            str(snapshot), str(context_path),
        ],
        cwd=root,
    )
    # The CLI adds a framing newline to stdout; the review digest covers the
    # actual JSON artifact bytes and therefore excludes that transport byte.
    if otio.endswith(b"\n"):
        otio = otio[:-1]
    (root / "review.otio").write_bytes(otio)
    bundle_value = json.loads(bundle)
    artifact_sha = bundle_value["review"]["identity"]["artifact_sha256"]
    if digest(otio) != artifact_sha:
        raise ValueError("review bundle artifact does not match the exported OTIO bytes")
    freshness = json.loads(
        invoke(
            [
                str(args.storytelling), "check-story-review", str(root / "review-bundle.json"),
                str(root / "proposal.json"), str(snapshot), str(context_path),
            ],
            cwd=root,
        )
    )
    if not freshness["freshness"]["current"]:
        raise ValueError("freshness check did not confirm the selected proposal artifact")

    wrapper = root / "delay-reply.py"
    wrapper.write_text(textwrap.dedent("""\
        import subprocess
        import sys
        import time
        from pathlib import Path

        host, data, empty, imported, otio, marker, stop = sys.argv[1:]
        command = [host, "--data-dir", data, "--project", empty, "--save-as", imported,
                   "import", otio]
        child = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        while child.poll() is None:
            if Path(stop).exists():
                child.kill()
                child.communicate()
                Path(marker).write_text("killed")
                sys.exit(124)
            time.sleep(0.02)
        stdout, stderr = child.communicate()
        Path(marker).write_text(str(child.returncode))
        time.sleep(300)  # Hold the acknowledgement after host exit to inject a lost reply.
        sys.stdout.buffer.write(stdout)
        sys.stderr.buffer.write(stderr)
        sys.exit(child.returncode)
    """))
    empty_project = root / "empty.fcproj"
    imported_project = root / "imported.fcproj"
    marker = root / "host-exit.marker"
    stop = root / "stop-host.signal"
    result_path = root / "journal-result.json"
    env = os.environ.copy()
    env.update({
        "M3_03_RUN_ROOT": str(root),
        "M3_03_FILMCRAFT": str(args.filmcraft),
        "M3_03_STORYTELLING": str(args.storytelling),
        "M3_03_PYTHON": str(args.python),
        "M3_03_DELAYED_REPLY_WRAPPER": str(wrapper),
        "M3_03_HOST_EXIT_MARKER": str(marker),
        "M3_03_STOP_SIGNAL": str(stop),
        "M3_03_RESULT_JSON": str(result_path),
    })
    test = subprocess.run(
        [
            str(args.cargo), "test", "--locked", "-p", "storytelling-journal",
            "--test", "filmcraft_execution", "--", "--ignored", "--exact",
            "isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback",
            "--nocapture",
        ],
        cwd=repo,
        env=env,
        capture_output=True,
    )
    (root / "logs/cargo-test.stdout").write_bytes(test.stdout)
    (root / "logs/cargo-test.stderr").write_bytes(test.stderr)
    if test.returncode:
        raise RuntimeError(
            "M3-03 integration proof failed; see " + str(root / "logs/cargo-test.stderr")
        )
    result = json.loads(result_path.read_bytes())
    if result["status"] != "passed" or result["retry_count"] != 0 or result["undo_count"] != 0:
        raise ValueError("proof did not satisfy the no-retry/no-Undo invariant")

    files = {
        str(path.relative_to(root)): {"bytes": path.stat().st_size, "sha256": digest(path.read_bytes())}
        for path in sorted(root.rglob("*"))
        if path.is_file() and path.name != "evidence.json"
    }
    evidence = {
        "schema_version": 1,
        "status": "passed",
        "profile": "filmcraft-25-picture-only-v1",
        "scope": result["scope"],
        "host": {
            "version": "filmcraft-cli 0.18.0",
            "binary_sha256": HOST_SHA256,
        },
        "storytelling_binary_sha256": digest(args.storytelling.read_bytes()),
        "ffmpeg_binary_sha256": digest(args.ffmpeg.read_bytes()),
        "python_version": invoke([str(args.python), "--version"], cwd=root).decode().strip(),
        "source_sha256": source_sha,
        "source_unchanged": result["source_unchanged"],
        "review_inputs_unchanged": result["review_inputs_unchanged"],
        "review_binding_sha256": result["review_binding_sha256"],
        "artifact_sha256": result["artifact_sha256"],
        "freshness_current": result["freshness_current"],
        "freshness_approval_granted": result["freshness_approval_granted"],
        "lost_ack_reconciliation": result["lost_ack_reconciliation"],
        "host_readback": result["host_readback"],
        "empty_project_sha256": result["empty_project_sha256"],
        "empty_project_unchanged": result["empty_project_unchanged"],
        "create_operation": result["create_operation"],
        "import_operation": result["import_operation"],
        "attempts": result["attempts"],
        "retry_count": result["retry_count"],
        "undo_count": result["undo_count"],
        "runner_sha256": digest(Path(__file__).read_bytes()),
        "rust_integration_test_sha256": digest(
            (repo / "crates/storytelling-journal/tests/filmcraft_execution.rs").read_bytes()
        ),
        "run_root": str(root),
        "artifacts": files,
    }
    (root / "evidence.json").write_text(json.dumps(evidence, indent=2) + "\n")
    print(json.dumps({"status": "passed", "evidence": str(root / "evidence.json"), "attempts": 1}))


if __name__ == "__main__":
    main()
