#!/usr/bin/env bash
set -euo pipefail

repo=$(cd "$(dirname "$0")/.." && pwd)
source_skill="$repo/skills/storytelling"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
export DISABLE_TELEMETRY=1 npm_config_ignore_scripts=true
skills=(npx --yes skills@1.8.1)

check_agents() {
  local installed=(.agents/skills/*)
  test "${#installed[@]}" -eq 1
  test "${installed[0]}" = .agents/skills/storytelling
  for target in .agents/skills/storytelling .claude/skills/storytelling; do
    diff -r "$source_skill/." "$target/."
  done
}

mkdir "$work/from-repo" "$work/from-npm" "$work/from-sync"

cd "$work/from-repo"
npm init --yes >/dev/null
# Select every discovered skill to catch duplicate or unintended entries.
"${skills[@]}" add "$repo" --skill '*' --agent codex claude-code --yes
check_agents

cd "$repo"
npm pack --silent --pack-destination "$work"
for project in from-npm from-sync; do
  cd "$work/$project"
  npm init --yes >/dev/null
  npm install --offline --save-dev --no-audit --no-fund "$work"/*.tgz
done

cd "$work/from-npm"
"${skills[@]}" add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
check_agents
test ! -L .agents/skills/storytelling
test ! -L .claude/skills/storytelling

cd "$work/from-sync"
sync_args=(experimental_sync --include @bynanci/storytelling --no-remote --agent codex claude-code --yes)
"${skills[@]}" "${sync_args[@]}" --no-frozen-lockfile
check_agents
cp skills-lock.json "$work/before-replay.json"
"${skills[@]}" "${sync_args[@]}" --frozen-lockfile
cmp skills-lock.json "$work/before-replay.json"
check_agents
