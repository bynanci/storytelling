# Security policy

<a name="安全政策"></a>

English · [繁體中文](SECURITY.zh-TW.md)

The project has no stable release. The local plan validator checks the metadata you supply. Its scope excludes sandboxing, network services and editor execution.

<a name="回報安全問題"></a>

## Report a security problem

Use GitHub private vulnerability reporting if the repository owner enabled it. Otherwise, ask the maintainer for a private channel. Keep vulnerability details, private media, local paths and credentials out of public issues. The owner controls private reporting and branch protection.

<a name="目前的安全邊界"></a>

## Current boundaries

The plan validator accepts up to 1 MiB, 1,024 assets and 4,096 shots. It rejects unknown fields, malformed numbers, duplicate IDs and missing references. It reads an explicit local file or standard input. It writes its response to standard output. This command cannot fetch source URLs, call a model, change a database or execute a host operation.

Repository rules prohibit `unsafe` in first-party Rust code. Review dependencies and compiler advisories separately.

The `verify-source` command compares up to 16 MiB of supplied bytes with an expected SHA-256 digest. A receipt describes only the bytes the caller supplied. A later read of the same path can return different content. The CLI does not resolve project URIs or establish the identity of a path. The isolated host tests generate authorized synthetic media in a fresh test directory. Each record documents its tested environment.

<a name="新增執行功能前"></a>

## Before adding execution

- Require explicit approval from a trusted source. Give each component only the permissions it needs. Model output and MCP annotations cannot grant authority.
- Check the source and the host. Prevent a stale owner from updating a later request.
- Keep durable receipts. Support cancellation. Read the actual host state when an operation has an unknown result.
- Isolate the host project. Remove paths and secrets from portable records. Exclude arbitrary shell commands and plug-ins.

For each finding, record the severity, affected version and reproduction conditions. Document the fix, regression test and remaining risk. Review relevant security and maintenance advisories even if an automated check reports no findings.
