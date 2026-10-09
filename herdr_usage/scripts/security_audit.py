#!/usr/bin/env python3
"""Offline source-policy checks. These do not replace runtime isolation or Rust tests."""
from pathlib import Path
import re
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
failures = []


def require(condition, message):
    if not condition:
        failures.append(message)


manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
allowed = {
    "anyhow", "clap", "crossterm", "directories", "serde", "serde_json",
    "thiserror", "time", "toml_edit", "zstd", "libc", "interprocess", "windows-sys",
}
runtime_dependencies = set(manifest.get("dependencies", {}))
for target in manifest.get("target", {}).values():
    runtime_dependencies.update(target.get("dependencies", {}))
require(runtime_dependencies <= allowed, "New runtime dependencies require a privacy review")
require(not (ROOT / "build.rs").exists(), "Review a new build.rs before allowing it")

for path in (ROOT / "src").rglob("*.rs"):
    # Production modules keep unit tests after cfg(test). Comments are not behavior.
    production = path.read_text(encoding="utf-8").split("#[cfg(test)]", 1)[0]
    code = re.sub(r"//[^\n]*", "", production)
    for pattern in [
        r"\b(?:TcpStream|TcpListener|UdpSocket|reqwest|ureq|hyper|tokio|TcpSocket)\b",
        r"std::net", r"libc::(?:socket|connect|send|sendto|sendmsg)\b",
        r'"(?:auth\.json|Authorization|Bearer|find-generic-password)"',
        r'Command::new\("(?:sh|bash|curl|wget|ssh|nc|codex|agy)"\)',
        r'"pane"\s*,\s*"read"',
        r"run_previous|run_shell_with_deadline|read_transcript_increment|enrich_cache_session",
    ]:
        require(not re.search(pattern, code), f"{path.relative_to(ROOT)}: forbidden runtime pattern {pattern}")
    for argument in re.findall(r"Command::new\(([^)]*)\)", code):
        require(argument.strip() in {'executable', '&executable', 'herdr', '"ps"'},
                f"{path.relative_to(ROOT)}: review new child command {argument}")

cache = (ROOT / "src/cache.rs").read_text(encoding="utf-8").split("#[cfg(test)]", 1)[0]
require('"transcript_path"' not in cache, "Transcript paths must not be persisted")
require("Value::Object(pc_obj.clone())" not in cache, "Do not copy an unrestricted prompt_cache object")
require("create_new(true)" in cache and "mode(0o600)" in cache, "Private atomic cache writes required")
require("from_mode(0o700)" in cache, "Private state directory required")
hook = (ROOT / "src/configure/agy.rs").read_text(encoding="utf-8").split("#[cfg(test)]", 1)[0]
require("MAX_STATUSLINE_BYTES" in hook, "Bound StatusLine input")
require("Command::" not in hook and "run_previous" not in hook, "StatusLine must not run another command")
require("parse_statusline(&observation" in hook, "Parse the sanitized StatusLine observation")
herdr = (ROOT / "src/herdr.rs").read_text(encoding="utf-8").split("#[cfg(test)]", 1)[0]
require('insert_optional_token(&mut tokens, "quota_topic"' not in herdr, "Never publish prompt topics")
require('for name in ["quota_topic", "quota_session", "quota_summary"]' in herdr,
        "Remove conversation metadata even when preserving old quota tokens")
for path in (ROOT.parent / ".github/workflows").glob("*.yml"):
    require(not re.search(r"WEBHOOK|\bcurl\b|\bwget\b", path.read_text(encoding="utf-8")),
            f"Outgoing repository relay: {path.name}")

if failures:
    print("Privacy source checks failed:", file=sys.stderr)
    for failure in failures:
        print(f"- {failure}", file=sys.stderr)
    sys.exit(1)
print("Privacy source checks passed (static policy; runtime and Rust validation still required).")
