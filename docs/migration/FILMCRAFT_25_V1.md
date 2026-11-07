# FilmCraft picture-only 25 fps proof v1 (M1-03)

Status: frozen after independent G2 review; actual execution evidence is separate.

The deliverable is a reproducible, test-only proof against the official FilmCraft
v0.18.0 Linux x86_64 CLI, plus bounded native verification of explicitly supplied
source bytes. It adds no host executor to the product. Rust remains 1.85 with
exactly the existing three implemented crates.

## Native supplied-buffer contract

`storytelling_core::source_evidence::verify_supplied_sha256(bytes: &[u8],
expected_sha256: &str) -> Result<VerifiedBufferV1, ValidationError>` accepts at most
16,777,216 bytes and exactly 64 lowercase hexadecimal expected SHA-256 digits.
Check order is byte budget, expected hash syntax, then digest equality. Empty
bytes are valid. Errors are `source.too_large` at `$`,
`source.invalid_expected_hash` at `$.expected_sha256`, and
`source.hash_mismatch` at `$.expected_sha256`; errors expose neither bytes, paths,
nor the observed digest.

Successful `VerifiedBufferV1` owns an immutable copy of those verified bytes.
All fields are private; the only creation path is verification. It has no
`Deserialize`, `Default`, public unchecked constructor, or mutable byte access.
Read-only getters expose bytes, byte length and lowercase SHA-256. Serialization
is metadata only: `schema_version: 1`, `algorithm: "sha256"`,
`scope: "provided-buffer-bytes"`, `byte_length`, `sha256`. A serialized report
cannot reconstruct a trusted buffer. This proves supplied bytes, not filesystem
identity, race-free future host reads, URI ownership, host support or approval.

The existing CLI consumer re-exports the API through the engine, keeping its
engine-only dependency boundary. `storytelling verify-source <FILE|-> <SHA256>`
reads only this explicit input (16 MiB + one byte maximum), never opens project
URIs, and returns `{"ok":true,"verification":<metadata>}`. Domain errors exit 1,
usage errors exit 2, input/output errors exit 3; success exits 0. Existing JSON
commands retain their 1 MiB input limit. Native core remains I/O-free.

Proposed dependency: exact `sha2 = 0.10.9`, default features disabled,
`force-soft` enabled. The task-specific review must assess the actual lockfile,
Rust 1.85 build, license and provenance before implementation is accepted. This
does not grant a general cryptography dependency exception.

## Host profile and independent oracle

Only 25 fps, picture-only, zero-origin media, no user effects, transitions, locks or
audio. Generate a two-second 1920×1080 25 fps video whose first second is red and
second is full-intensity green (`0x00ff00`), plus a blue 1920×1080 PNG. Bind actual source SHA-256 and sizes
using the native verifier before and after the host run.

The source composition contains three ordered occurrences on one picture track:

| Occurrence | Source | Source range | Timeline range | Frames |
|---|---|---|---|---|
| red-trim | reused video | [0.04, 0.84) s | [0, 0.8) s | 20 |
| blue-hold | blue still | [0, 0.2) s | [0.8, 1) s | 5 |
| green-repeat | same video | [1, 1.8) s | [1, 1.8) s | 20 |

M1-02 must emit common OTIO clock 25; high common clocks are outside this host
profile because FilmCraft can infer sequence fps from the first duration rate.
Expected native settings are 1920×1080 at 25/1 fps, exact ticks per second
254,016,000,000, 45 frames and duration 457,228,800,000 ticks. Read exact native
JSON and inspect data, including source ranges, media type, native item/clip IDs,
speed, reverse, effects and transitions. Require only default identity intrinsic
motion/opacity/time-remap effects (no masks or keyframes). FilmCraft allocates fresh IDs and drops
foreign metadata: retain a separate explicit source occurrence → native clip →
native media mapping, never claim ID preservation.

The checked CPU encoded preview is H.264/YUV420p at 192×108 (`--scale 0.1`);
native sequence and input sources remain 1920×1080. This certifies that explicit
preview scale, not a full-resolution export.

Create a new empty project, hash it, import the real M1-02 OTIO into a distinct
project, inspect, CPU render/export an encoded movie, save as a third native
project, reopen, export OTIO and reimport that into another new empty project.
Require exit success *and* JSON `errors: []`, no offline media, no unexplained
warnings/loss report entries, and exact native ranges/settings. Independent
FFprobe verifies encoded duration, fps, frame count and timestamps; decoded
RGB means verify 20 red, five blue, 20 green frames (dominant channel >180 and
others <70, allowing codec conversion). Colors prove order and hold lengths;
exact native ticks separately prove trim offsets. No `--allow-offline` flag is permitted.

All test projects, logs, synthetic media, XDG directories and `--data-dir` live
under a newly owned run root. Do not change `HOME` or launch a user project.
This is output confinement by explicit test paths, not an OS filesystem jail.
Require original empty project, imported/saved project inputs and media to stay
unchanged across read-only reopen/export steps. Record every command, exit code,
stdout/stderr hash, source/output hash, binary version/hash and release digest.

No claim covers audio, GUI/GPU, arbitrary OTIO, high clocks, real user media,
human editorial acceptance, full source filesystem TOCTOU closure, or all 64
ports. Only actually observed stages may be marked complete.
