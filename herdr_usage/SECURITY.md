# Security and offline operation

## Runtime boundary

The fork collects Codex metrics from local rollout files and Agy metrics from
StatusLine stdin. It has no HTTP client, credential-store reader, telemetry
endpoint or provider CLI subprocess on its update paths. Herdr communication
uses the configured local CLI and Unix sockets or Windows named pipes; it is not a TCP connection.

The plugin does not read pane output or follow transcript paths supplied by
StatusLine. It does not publish prompt topics or session summaries. Codex rollouts
contain conversations as well as metrics: bounded file reads necessarily bring
some conversation bytes into process memory, but only technical metrics and
session/model identifiers are retained. No rollout/transcript file is uploaded.

Agy input is bounded to 1 MiB. The hook parses an allowlist of typed fields;
unknown fields and nested prompt/cache content are discarded. The hook is silent
and never executes a previous StatusLine command. Configuration keeps that
command for restoration on uninstall, so its visual output is suspended while
the collector is installed.

## Local state

State contains usage windows, token/cache counters, models, technical session/pane
IDs, preferences, attention state and watcher coordination. Sessions remain local;
this is not a promise that no session identifier is stored.

On Unix, cache directories are restricted to 0700 and newly replaced snapshots,
observations, backups and Agy settings files to 0600. Atomic scratch creation
refuses existing files or symlinks. A state-directory symlink is rejected.

Configuration reads Agy's complete settings JSON and Herdr's configuration to
preserve unrelated settings. The original Herdr configuration is backed up locally.
These backups can contain a literal secret if the original configuration or
StatusLine command already contained one. They are retained locally for exact restoration; the previous command is not
read or executed by the StatusLine update hook. Avoid placing secrets in shell
command literals. Existing legacy state is not automatically erased by an upgrade.

## Trust and verification limits

Use trusted Herdr/agent binaries and local, private filesystem paths. Environment
variables can override binary paths, socket paths, rollout roots and state paths;
a compromised binary, remote-mounted directory, cloud backup, administrator or
another process running as the same user is outside this code's isolation boundary.
The plugin cannot prevent network requests made independently by Codex, Agy, Herdr
or the operating system.

Offline collection shows only metrics already recorded locally. It cannot obtain
new quota observations without a producer writing them. Installation/build tooling
can download a toolchain or dependencies; preinstall the pinned toolchain and cache
dependencies before using `cargo build --release --locked --offline`.

Windows watchers start without inheriting parent capture handles. Native process
attribution queries creation time with limited rights, without reading process memory.

Run `python3 scripts/security_audit.py`, then the Rust checks in CONTRIBUTING.md.
The static check detects policy regressions; it is not a proof of total security.
For reviewed findings and unperformed checks, see [PRIVACY_AUDIT.md](docs/PRIVACY_AUDIT.md).
Do not publish credentials, secrets or private conversations in bug reports.
