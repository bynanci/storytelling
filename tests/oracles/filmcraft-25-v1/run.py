# SPDX-License-Identifier: Apache-2.0
"""Isolated official FilmCraft v0.18.0 picture-only proof; no product executor."""
import argparse
import hashlib
import json
import os
import re
from fractions import Fraction
from pathlib import Path
import subprocess

HOST_SHA256 = "064a1f55cde5f6e1ac04bcd3a5b72e5d7366905f3d7333793e78dda0c109d33f"
TICKS = 254_016_000_000
EXPECTED = [("red-trim", "reused-video", 0, 20, 1, 21),
            ("blue-hold", "blue-still", 20, 25, 0, 5),
            ("green-repeat", "reused-video", 25, 45, 25, 45)]
COLORS = ["red"] * 20 + ["blue"] * 5 + ["green"] * 20


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def shape(data, keys):
    require(isinstance(data, dict) and set(data) == set(keys), "unexpected command response fields")


def check_import(data):
    shape(data, ("errors", "items", "sequences", "documents"))
    require(data["errors"] == [], "import errors")
    require(data["items"] == [] and len(data["sequences"]) == 1, "import shape")
    require(len(data["documents"]) == 1, "one import document")
    doc = data["documents"][0]
    shape(doc, ("format", "linkedMedia", "offlineMedia", "report", "sequences"))
    require(doc["format"] == "OpenTimelineIO" and doc["linkedMedia"] == 2, "linked media")
    require(doc["offlineMedia"] == [] and doc["report"] == [], "offline media or losses")
    require(doc["sequences"] == data["sequences"], "import sequence identity")


def check_colors(data, expected):
    require(len(data) == 3 * len(expected), "decoded frame count")
    channel = {"red": 0, "green": 1, "blue": 2}
    means = []
    for index, name in enumerate(expected):
        rgb = list(data[index * 3:index * 3 + 3])
        require(rgb[channel[name]] > 180 and all(v < 70 for j, v in enumerate(rgb) if j != channel[name]), f"frame {index}: expected {name}, got {rgb}")
        means.append(rgb)
    return means


def check_sequence(data):
    require(data["duration"] == TICKS * 9 // 5 and data["durationFrames"] == 45, "native duration")
    settings = data["settings"]
    require(settings["width"] == 1920 and settings["height"] == 1080 and settings["frame_rate"] == {"num": 25, "den": 1}, "native settings")
    require(data["markers"] == [] and len(data["video"]) == 1, "sequence shape")
    require(all(t["items"] == [] and t["transitions"] == [] for t in data["audio"]), "audio not empty")
    track = data["video"][0]
    require(track["enabled"] is True and track["locked"] is False and track["muted"] is False and track["transitions"] == [], "track state")
    clips = track["items"]
    require(len(clips) == 3 and len({c["clip"] for c in clips}) == 3, "three distinct occurrences")
    require(clips[0]["item"] == clips[2]["item"] != clips[1]["item"], "repeated media identity")
    neutral = {
        "motion": {"anchor": "Vec2(Vec2 { x: 960.0, y: 540.0 })", "position": "Vec2(Vec2 { x: 960.0, y: 540.0 })", "anti_flicker": "Float(0.0)", "rotation": "Float(0.0)", "scale": "Float(100.0)", "scale_width": "Float(100.0)", "uniform_scale": "Bool(true)"},
        "opacity": {"blend": "Choice(0)", "opacity": "Float(100.0)"},
        "time_remap": {"speed": "Float(100.0)"},
    }
    mapping = []
    for clip, (occurrence, asset, start, end, sin, sout) in zip(clips, EXPECTED):
        require([clip[k] for k in ("startFrame", "endFrame", "durationFrames")] == [start, end, end - start], "native frame ranges")
        require([clip[k] for k in ("start", "end", "duration", "sourceIn", "sourceOut")] == [TICKS * v // 25 for v in (start, end, end - start, sin, sout)], "native tick ranges")
        require(clip["speed"] == 1 and clip["reverse"] is False and clip["enabled"] is True and clip["gainDb"] == 0 and clip["link"] is None, "clip state")
        require(len(clip["effects"]) == 3 and {e["effect"] for e in clip["effects"]} == set(neutral), "intrinsic effects only")
        for effect in clip["effects"]:
            require(effect["enabled"] is True and effect["masks"] == 0, "intrinsic effect state")
            require({k: p["value"] for k, p in effect["params"].items()} == neutral[effect["effect"]], "neutral intrinsic parameters")
            require(all(p["keyframes"] == 0 for p in effect["params"].values()), "no keyframes")
        mapping.append({"source_occurrence_id": occurrence, "source_asset_id": asset, "native_clip_id": clip["clip"], "native_media_id": clip["item"], "timeline_frames": [start, end], "source_frames": [sin, sout]})
    return mapping


def check_native(data, mapping, assets):
    require(data["format"] == "filmcraft.project" and data["schema_version"] == 17, "native schema")
    items = data["project"]["items"]
    require(len(items) == 3, "native sequence plus two media")
    for asset_id, filename, kind in assets:
        item_id = next(m["native_media_id"] for m in mapping if m["source_asset_id"] == asset_id)
        media = items[str(item_id)]["kind"]["Media"]
        require(media["media"]["File"]["path"] == str(filename), "native source path")
        require(media["info"]["kind"] == kind and media["offline"] is False and media["proxy"] is None, "native media status")
        require(media["info"]["audio_streams"] == [] and media["info"]["video"]["width"] == 1920 and media["info"]["video"]["height"] == 1080, "native probed media")
        require(media["identity"]["size"] == filename.stat().st_size and media["identity"]["fingerprint"], "native identity populated")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--storytelling", type=Path, required=True)
    parser.add_argument("--filmcraft", type=Path, required=True)
    parser.add_argument("--ffmpeg", type=Path, default=Path("/usr/bin/ffmpeg"))
    parser.add_argument("--ffprobe", type=Path, default=Path("/usr/bin/ffprobe"))
    parser.add_argument("--run-root", type=Path, required=True)
    args = parser.parse_args()
    root = args.run_root
    require(root.is_absolute() and not root.exists(), "run root must be new and absolute")
    for tool in (args.storytelling, args.filmcraft, args.ffmpeg, args.ffprobe):
        require(tool.is_absolute() and tool.is_file(), "explicit executable missing")
    require(digest(args.filmcraft.read_bytes()) == HOST_SHA256, "unverified FilmCraft executable")
    root.mkdir(parents=True)
    (root / "logs").mkdir()
    env = os.environ.copy()
    for kind in ("CONFIG", "DATA", "CACHE", "STATE"):
        env[f"XDG_{kind}_HOME"] = str(root / f"xdg-{kind.lower()}")
    env["FILMCRAFT_DEV_DESKTOP_ENTRY"] = "0"
    commands = []

    def run(argv, stderr_ok=False):
        argv = [str(a) for a in argv]
        p = subprocess.run(argv, capture_output=True, env=env, cwd=root, timeout=120)
        number = len(commands)
        (root / "logs" / f"{number:03}-stdout").write_bytes(p.stdout)
        (root / "logs" / f"{number:03}-stderr").write_bytes(p.stderr)
        commands.append({"argv": argv, "exit": p.returncode, "stdout_sha256": digest(p.stdout), "stderr_sha256": digest(p.stderr), "stderr": p.stderr.decode(errors="replace")})
        require(p.returncode == 0, f"command {number} failed: {p.stderr.decode(errors='replace')}")
        require(stderr_ok or not p.stderr, f"unexpected stderr command {number}")
        if stderr_ok and p.stderr:
            require(re.fullmatch(r"exported " + re.escape(str(root / "movie.mp4")) + r" in [0-9]+(?:\.[0-9]+)?s\n", p.stderr.decode()) is not None, "unexpected export warning on stderr")
        return p.stdout

    def host(project, *argv, stderr_ok=False):
        command = [args.filmcraft, "--data-dir", root / "data"]
        if project is not None:
            command += ["--project", project]
        return run(command + list(argv), stderr_ok=stderr_ok)

    version = run([args.filmcraft, "--version"]).decode().strip()
    require(version == "filmcraft-cli 0.18.0", "host version")
    tool_versions = {str(args.filmcraft): version}
    for tool, flag in ((args.storytelling, "--version"), (args.ffmpeg, "-version"), (args.ffprobe, "-version")):
        tool_versions[str(tool)] = run([tool, flag]).decode().splitlines()[0]
    video, blue = root / "source.mp4", root / "blue.png"
    run([args.ffmpeg, "-v", "error", "-f", "lavfi", "-i", "color=0xff0000:s=1920x1080:r=25:d=1", "-f", "lavfi", "-i", "color=0x00ff00:s=1920x1080:r=25:d=1", "-filter_complex", "[0:v][1:v]concat=n=2:v=1:a=0[v]", "-map", "[v]", "-c:v", "libx264", "-pix_fmt", "yuv420p", video])
    run([args.ffmpeg, "-v", "error", "-f", "lavfi", "-i", "color=0x0000ff:s=1920x1080:r=25", "-frames:v", "1", blue])

    def colors(filename, expected):
        raw = run([args.ffmpeg, "-v", "error", "-i", filename, "-vf", "scale=1:1:flags=area", "-pix_fmt", "rgb24", "-f", "rawvideo", "-"])
        return check_colors(raw, expected)

    colors(video, ["red"] * 25 + ["green"] * 25)
    colors(blue, ["blue"])
    assets = [("reused-video", video, "Movie"), ("blue-still", blue, "Still")]
    sources = []
    wire_assets = []
    for aid, filename, kind in assets:
        expected = digest(filename.read_bytes())
        receipt = json.loads(run([args.storytelling, "verify-source", filename, expected]))
        require(receipt == {"ok": True, "verification": {"schema_version": 1, "algorithm": "sha256", "scope": "provided-buffer-bytes", "byte_length": filename.stat().st_size, "sha256": expected}}, "native source receipt")
        sources.append({"asset_id": aid, "path": str(filename), "sha256": expected, "verification_before": receipt["verification"]})
        wire = {"id": aid, "uri": filename.as_uri(), "mediaType": "video" if kind == "Movie" else "image", "name": aid, "contentHash": expected, "tags": [], "state": {}, "metadata": {}}
        if kind == "Movie":
            wire.update(duration=2, metadata={"openfilm.ffprobe": {"streams": [{"codec_type": "video"}]}})
        wire_assets.append(wire)
    clips = [{"id": occ, "assetId": aid, "timelineStart": start / 25, "timelineDuration": (end - start) / 25, **({"sourceIn": sin / 25, "sourceOut": sout / 25} if aid == "reused-video" else {})} for occ, aid, start, end, sin, sout in EXPECTED]
    snapshot = {"schemaVersion": "1.0.0", "composition": {"id": "filmcraft-25-proof", "storyId": "authorized-synthetic", "duration": 1.8, "tracks": [{"id": "picture", "type": "video", "clips": clips}]}, "assets": wire_assets, "settings": {"width": 1920, "height": 1080, "frameRate": 25}}
    source_json, otio = root / "source.json", root / "input.otio"
    source_json.write_text(json.dumps(snapshot, indent=2) + "\n")
    compiled = run([args.storytelling, "export-otio", source_json, "--picture-only"])
    document = json.loads(compiled)
    require(document["metadata"]["storytelling"]["report"]["content_clock_rate"] == "25", "OTIO common clock")
    otio.write_bytes(compiled)
    empty, imported, saved, reimported = [root / n for n in ("empty.fcproj", "imported.fcproj", "saved-again.fcproj", "reimported.fcproj")]
    new = json.loads(host(None, "--save-as", empty, "exec", "file.newProject", "name=Review"))
    shape(new, ("name", "revision"))
    require(new["name"] == "Review", "new project")
    empty_project = json.loads(host(empty, "inspect", "project"))
    require(empty_project["activeSequence"] is None, "empty project has active sequence")
    unchanged = {p: digest(p.read_bytes()) for p in (empty, source_json, otio, video, blue)}
    imported_result = json.loads(host(empty, "--save-as", imported, "import", otio))
    check_import(imported_result)
    sequence = json.loads(host(imported, "inspect", "sequence"))
    mapping = check_sequence(sequence)
    check_native(json.loads(imported.read_bytes()), mapping, assets)
    unchanged[imported] = digest(imported.read_bytes())
    movie = root / "movie.mp4"
    rendered = json.loads(host(imported, "export", movie, "--format", "h264", "--gpu-rendering", "off", "--range", "entire", "--no-audio", "--scale", "0.1", stderr_ok=True))
    shape(rendered, ("job", "offlineMedia", "path", "result", "warnings"))
    shape(rendered["result"], ("bytes", "frames", "path", "render_fps", "seconds"))
    require(rendered["warnings"] == [] and rendered["offlineMedia"] == [] and rendered["result"]["frames"] == 45 and rendered["path"] == str(movie), "CPU movie result")
    save_result = json.loads(host(imported, "exec", "file.saveAs", "path=" + str(saved)))
    shape(save_result, ("backup", "bytes", "ms", "path"))
    require(save_result["path"] == str(saved) and save_result["bytes"] == saved.stat().st_size, "native save result")
    reopened = json.loads(host(saved, "inspect", "sequence"))
    require(check_sequence(reopened) == mapping and reopened == sequence, "native reopen identity")
    check_native(json.loads(saved.read_bytes()), mapping, assets)
    unchanged[saved] = digest(saved.read_bytes())
    readback = root / "readback.otio"
    exported = json.loads(host(saved, "exec", "file.exportInterchange", "format=otio", "path=" + str(readback)))
    shape(exported, ("bytes", "path", "report"))
    require(exported["report"] == [] and exported["bytes"] == readback.stat().st_size and exported["path"] == str(readback), "OTIO export result")
    roundtrip = json.loads(host(empty, "--save-as", reimported, "import", readback))
    check_import(roundtrip)
    roundtrip_sequence = json.loads(host(reimported, "inspect", "sequence"))
    roundtrip_mapping = check_sequence(roundtrip_sequence)
    check_native(json.loads(reimported.read_bytes()), roundtrip_mapping, assets)
    probe = json.loads(run([args.ffprobe, "-v", "error", "-show_streams", "-show_format", "-show_frames", "-of", "json", movie]))
    require(len(probe["streams"]) == 1, "encoded video only")
    stream = probe["streams"][0]
    require(stream["codec_name"] == "h264" and stream["pix_fmt"] == "yuv420p" and stream["width"] == 192 and stream["height"] == 108, "encoded format")
    require(stream["avg_frame_rate"] == "25/1" and stream["r_frame_rate"] == "25/1" and int(stream["nb_frames"]) == 45, "encoded fps/frame count")
    require(Fraction(stream["duration"]) == Fraction(9, 5) and Fraction(probe["format"]["duration"]) == Fraction(9, 5), "encoded duration")
    timestamps = [f["best_effort_timestamp_time"] for f in probe["frames"]]
    require(len(timestamps) == 45 and all(Fraction(t) == Fraction(i, 25) for i, t in enumerate(timestamps)), "encoded presentation timestamps")
    rgb = colors(movie, COLORS)
    for source in sources:
        receipt = json.loads(run([args.storytelling, "verify-source", source["path"], source["sha256"]]))
        require(receipt["verification"] == source["verification_before"], "source changed after host")
        source["verification_after"] = receipt["verification"]
    for path, before in unchanged.items():
        require(digest(path.read_bytes()) == before, "original input changed: " + path.name)
    files = {str(p.relative_to(root)): {"sha256": digest(p.read_bytes()), "bytes": p.stat().st_size} for p in root.rglob("*") if p.is_file()}
    evidence = {"schema_version": 1, "profile": "filmcraft-25-picture-only-v1", "status": "passed", "scope": "synthetic CPU headless integration proof, not host approval or general OTIO support", "host": {"repository": "https://github.com/storytold/filmcraft", "release": "v0.18.0", "source_commit": "015d5b3ea37e1202a529e6245d3cc021c8c16ffd", "archive_sha256": "0179f05b20db855bfd3def9136ce60dee59f22d2dcbfcf23e4a6256e0c153b05", "binary_sha256": HOST_SHA256, "version": version}, "tools": {str(t): digest(t.read_bytes()) for t in (args.storytelling, args.ffmpeg, args.ffprobe)}, "sources": sources, "source_to_native_mapping": mapping, "readback_to_native_mapping": roundtrip_mapping, "native_settings": sequence["settings"], "native_duration_ticks": sequence["duration"], "encoded": {"codec": stream["codec_name"], "pixel_format": stream["pix_fmt"], "frames": 45, "fps": "25/1", "duration_seconds": stream["duration"], "size": [192, 108], "presentation_timestamps": timestamps, "decoded_rgb_means": rgb}, "checked_stages": ["new-empty-project", "native-source-verification-before", "real-OTIO-import", "exact-native-ticks-and-identity-effects", "CPU-encoded-movie", "native-save-as", "native-reopen", "OTIO-export", "OTIO-reimport", "independent-FFprobe-and-decoded-colors", "native-source-verification-after", "input-immutability"], "unchanged_inputs": {str(p): h for p, h in unchanged.items()}, "run_root": str(root), "commands": commands, "artifacts": files}
    repo = Path(__file__).resolve().parents[3]
    def git(*argv):
        return subprocess.check_output(["git", "-C", str(repo), *argv])
    evidence["candidate"] = {"head": git("rev-parse", "HEAD").decode().strip(), "base_tree": git("rev-parse", "HEAD^{tree}").decode().strip(), "tracked_status": git("status", "--porcelain", "--untracked-files=no").decode(), "tracked_diff_sha256": digest(git("diff", "HEAD", "--binary")), "harness_sha256": digest(Path(__file__).read_bytes()), "negative_checks_sha256": digest(Path(__file__).with_name("test_checks.py").read_bytes()), "binary_binding": "actual executable SHA-256 above; receipt may be committed afterward as documentation-only changes"}
    evidence["tool_versions"] = tool_versions
    evidence["environment"] = {"selected_runtime_overrides": {key: env[key] for key in ["XDG_CONFIG_HOME", "XDG_DATA_HOME", "XDG_CACHE_HOME", "XDG_STATE_HOME", "FILMCRAFT_DEV_DESKTOP_ENTRY"]}, "home_unchanged": env.get("HOME") == os.environ.get("HOME"), "inherited_environment_recorded": False, "note": "Inherited proxy/CA/secrets are not collected; no environment dump or redaction-derived proof."}
    evidence["preview_scale"] = {"factor": "0.1", "native_width": 1920, "native_height": 1080, "encoded_width": 192, "encoded_height": 108}
    evidence["host"]["provisioning"] = {"archive_url": "https://github.com/storytold/filmcraft/releases/download/v0.18.0/filmcraft-0.18.0-linux-x86_64.tar.gz", "checksum_manifest_sha256": "2c231596c95e19ddc4e24e89c1f9ef073734f52fb334b0dd0f8118f4b2ca043c", "prior_checks": ["official archive asset digest", "official SHA256SUMS archive row", "official SHA256SUMS asset digest", "archive members inspected before extraction; no links/path escapes"], "detached_signature_verified": False, "note": "Recorded prior provisioning evidence, not rerun by this harness; every invocation independently verifies extracted binary SHA-256."}
    (root / "evidence.json").write_text(json.dumps(evidence, indent=2) + "\n")
    print(json.dumps({"status": "passed", "evidence": str(root / "evidence.json"), "frames": 45}))


if __name__ == "__main__":
    main()
