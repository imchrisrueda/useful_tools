# Agent guide

Notes for agents working on `herdr-agent-usage`. Read this before touching
anything that talks to Herdr.

## Working method

1. Establish the exact requested scope and inspect the current diff before
   editing. Treat unrelated worktree changes as user-owned.
2. Separate observed facts, inferences, and unknowns. When evidence is missing,
   name the cheapest useful verification instead of guessing.
3. Prefer the smallest surgical change that creates a checkable behavior. Add
   abstractions only when two real callers or adapters need the same seam.
4. Give every implementation step a verification condition and run the
   repository gates before calling it complete.
5. For multi-goal work, keep the decision record and dispatch prompts under
   the ignored `.agents/` directory. Public documentation must describe shipped
   behavior, not private execution state.

Before changing dependencies, inspect `Cargo.toml`, `Cargo.lock`, and
`rust-toolchain.toml`. Use the pinned Rust toolchain and repository-local Cargo
artifacts; do not install project tooling globally.

## The rule that matters most: reading or writing a pane is not free

No entry point may read pane output. Prompt topics are disabled for privacy.
Never execute a saved StatusLine command or follow a StatusLine transcript path.
Metadata writes still carry repaint risk, so avoid no-op writes. Remove legacy
conversation tokens during the normal publish pass without adding a second pass.

For scrolling reports or changes to pane-read behavior, read
[pane repaint diagnosis](docs/pane-repaint-diagnosis.md) before probing.
Scroll offsets and before/after content hashes cannot detect the transient repaint;
use the documented human observation rather than polling live panes.

Concretely, this means:

1. **Never read terminal output.** Events resolve only their named pane using
   metadata. Usage collection must not inspect prompts or terminal transcripts.
2. **Publish once per invocation.** Two `publish` passes in a row means each
   pane can take two metadata writes for one user action.
3. **Keep `metadata_matches` honest** (`src/herdr.rs`). It is the only thing
   stopping a no-op refresh from repainting every pane. If you add a token,
   add it to `METADATA_TOKEN_NAMES` too, or the comparison silently stops
   covering it and every refresh becomes a write.
4. **Preserve verified metrics.** Missing readings must not manufacture zeros.
   Prompt topics and legacy conversation summaries are intentionally removed for privacy;
   they must never be reintroduced by a cache or metadata preservation path.

## Event paths, and what each is allowed to do

| Entry point | Fired by | Allowed to read panes? |
|---|---|---|
| `startup` | Herdr's `[[startup]]` hook | No |
| `refresh` | manual action, `startup` | No |
| `event` | `pane.agent_detected`, `pane.agent_status_changed` | No (only named-pane metadata) |
| `focus` | `pane.focused`, `workspace.focused`, `tab.focused` | No |
| `watch` | detached from a working status event | No (agent metadata only) |

`startup` exists because Herdr drops plugin-owned Agent views when the server
exits, and startup hooks run again after a restart or a live handoff. It
restores plugin-owned views, forces one quota refresh, and restores the watcher.
Plugin enable alone does not run startup; the configure action runs it after
repair. Server-owned event/refresh paths also record the current Herdr binary
and socket so an older watcher can adopt the new connection.

`pane.agent_status_changed` fires **twice per turn** (idle→working on submit,
working→idle on completion). Anything `event` does, the user pays for twice
every time they press Enter. Budget accordingly.

The working event starts one global `watch` pulse. It calls `herdr agent list`
once per configured interval for every supported harness (Codex and Agy). Event-spawned watchers defer their first poll. They resolve local
billing targets, refresh active/settling targets, and publish to siblings with
the same target without reading terminal output. A finishing target stays in
the pass until the 60-second debounce has elapsed. The interval defaults to
60 seconds and is bounded to 30 seconds–1 hour. While a pane is working or
has an unseen completion, the watcher also checks the metadata-only Herdr
snapshot once per second. Herdr 0.9 can miss TUI focus hooks; the snapshot
reconciles those changes without reading pane output or writing unchanged
metadata. The watcher stays alive for unseen completions until they are seen.
Local stop/connection checks interrupt sleeps. Uninstall writes a stop marker.

## Local-first metrics: Codex and Antigravity (Agy)

Update paths operate locally without credential-store access or remote API requests.
Configuration backups may retain secrets already present in user settings; see SECURITY.md.

1. **Codex**: Quota windows (5h, 7d), token usage, context window, and model are parsed
   directly and locally from rollout files (`.jsonl` and compressed `.jsonl.zst`) in
   `~/.codex/sessions`. No `codex app-server` or background child processes are spawned.
   Session attribution matches the pane's foreground cwd and process start time to rollout `session_meta`.
2. **Antigravity (Agy)**: StatusLine IPC hook writes structured JSON payloads to a dedicated
   mailbox file under the plugin state directory. Quota windows (`5h`, `7d`, and third-party `api` pools on Gemini),
   model display name, context window percentages, and spending pace are extracted directly with zero external credentials.

## Quota attribution and cache rules

- Codex rollouts provide local-only session windows and diagnostics without contacting external endpoints.
- Agy StatusLine observations carry `session_quota_only`; they are never merged or shared across sessions.
- Agy must identify the active pool or receive only one possible pool. Do not combine Gemini and third-party quotas for an unknown model.
- Failed reads or missing data preserve the last verified observation; they never manufacture zero usage or guessed data.

## Herdr state this plugin owns outside a pane

Two things reach past the pane metadata, and both are global to the Herdr
session rather than scoped to a pane. Low-quota notifications stay off until
the user sets a threshold. The Agent view is on by default (`--agent-order
quota`): Space grouping plus least-headroom ranking inside each space.

**The Agent view** (`agent.view.set`, `src/herdr.rs`). Herdr keeps exactly
one, and setting it replaces the user's own `ui.agent_panel_sort`. Rules:

1. **Always scope a clear to `plugin:herdr-agent-usage`.** An unscoped
   `agent.view.clear` would drop a view another plugin owns. `startup` goes
   further and does not call clear at all when the order is `default` — there
   is nothing of ours to restore, and silence is the only way to be sure a
   foreign view survives.
2. **Re-apply it from `startup` and a forced refresh, never from event.**
   Herdr drops a plugin-owned view on disable; enable does not run startup.
   The refresh action (`--force`) is the same repair that respawns the
   watcher. Event/focus/watch stay off this path so a turn does not spend a
   socket call.
3. It is the only thing in the plugin that speaks the raw socket protocol
   (`HERDR_SOCKET_PATH`), because `agent.view.*` has no CLI subcommand in
   Herdr 0.8. One request, one reply, one connection — nothing subscribes, so
   the `events.subscribe` replay and focus-storm problems do not apply.
4. **Quota order keeps Spaces contiguous.** The sort is
   `workspace_order` ascending, then `quota_headroom` ascending — never a
   flat headroom list that scatters one project's panes across the panel.
   `$quota_group` names the Space on the tightest pane in that workspace;
   `$quota_icon` is the vendor mark on every identity row (bundled icon font).
   Working/done colour is an invisible suffix matched
   by sidebar `rules`, not a later twin token — a later `$quota_icon_done`
   hang-indents one cell under the Space name. Colour replaces Herdr's `state_icon`
   ring: yellow while working, teal for an unseen completion, white after
   focusing that pane or moving focus away from it. Do not trust CLI `agent_status` for the teal
   step — same-tab siblings finish as server `idle` while the TUI ring is
   still teal. Persist working/unseen pane ids in plugin state
   (`icon-attention.json`) and never call `herdr pane current` from
   `event`: status hooks set `HERDR_PANE_ID` to the finisher. Focus hooks
   mark only the previous and newly focused panes seen. A workspace or Tab
   switch may not emit `pane.focused`; resolve its pane from that location's
   layout in `herdr api snapshot`. Ignore a delayed event whose workspace or
   Tab is no longer focused.

**`quota_headroom`** is the token that view sorts on: the remaining percent of
the tightest of the pane's 5h, 7d, and 30d windows, zero-padded to three digits
so Herdr's ordering of the text is its numeric ordering. Two properties are
load bearing:

- It is published **unconditionally**, not only when the order is enabled. No
  sidebar row renders it, so it costs no screen space; publishing it always is
  what makes toggling the order a Herdr-side change instead of a metadata write
  to every pane, and it adds no writes, because it only moves when a quota
  token beside it moves anyway.
- It is scoped to the windows the sidebar actually **shows**. A window without
  a token never decides the sort or an alert.

**Low quota notifications** fire from both publish paths (`publish_resolved`
and `handle_named_pane`) so a warning lands at the end of the turn that spent
the quota. The state is a set of provider names, not a timestamp: a provider
stays quiet while it stays low and is re-armed only by recovering above the
threshold. A provider with **no pane in the pass keeps its entry** — dropping
it would make closing and reopening a pane a way to be warned twice.

## A plugin action cannot see the caller's environment

Herdr runs `[[actions]]` with a fixed command line **in the server's own
environment**. A variable exported around `herdr plugin action invoke` does not
reach the action. Measured with a temporary `printenv` action: of 61 variables,
the only Herdr-related ones present were `HERDR_PLUGIN_STATE_DIR` and
`HERDR_PLUGIN_CONFIG_DIR`, both injected by Herdr; neither the probe marker nor
`HERDR_AGENT_QUOTA_AGENTS` survived.

So `src/prefs.rs` — small files under `HERDR_PLUGIN_CONFIG_DIR` — is the only
channel an installer has for passing a choice to `configure`. Environment
variables still work for a **direct CLI run** and are read first, but anything
that must survive `install.sh` / `uninstall.sh` has to be written as a
preference. This bit once: `./uninstall.sh --agent codex` passed the selection
through `env`, it never arrived, and the default selection is *every* agent, so
a partial uninstall removed everything.

To re-check this on a new Herdr version, append a throwaway action running
`printenv > /tmp/probe.txt`, reload with `herdr plugin disable && herdr plugin
enable`, invoke it with a marker variable set, and read the file.

## Event payload shapes

`HERDR_PLUGIN_EVENT_JSON` is nested and not uniform across events. `pane.focused`
carries no `agent`: `focus` uses its pane ID and resolves the harness from one
agent inventory read. Only a direct `focus` invocation without event JSON uses
`herdr pane current`. Workspace and Tab focus events carry no pane ID, so
`focus` resolves the matching layout from `herdr api snapshot`. This keeps
delayed events from redirecting a refresh to another pane:

```json
{"event":"pane_focused","data":{"type":"pane_focused","pane_id":"w1:p9","workspace_id":"w1"}}
```

`find_agent` and `find_pane_id` in `src/refresh.rs` walk the tree rather than
assuming a fixed path. Keep them tolerant — the shapes differ per event and are
not part of a stable contract.

## Adding a harness

Append to `AgentSelection::SUPPORTED`. Never insert. A saved complete agent
list is a proper prefix of that array (`[Harness::Codex, Harness::Agy]`).
Settings and `install.sh --agent` write `all` or `only,<names>`.

Wiring a new name requires:

1. `Harness`, `from_agent_name`, `AgentSelection` (the enum, `parse`,
   `harness`, `harness_name`), clap `--agent` help, `install.sh` comments,
   both READMEs, and the plugin description.
2. A `PROVIDER_STYLES` row in `src/configure/herdr.rs`, in `SUPPORTED` order.
3. Settings popup `height` in `herdr-plugin.toml` — one more row. The
   `rows().len()` check fails if this is skipped.
4. If it has a quota collector: `Provider`, `Provider::ALL`,
   `ProviderSelection`, the fetch path, and a cache identity. If Herdr has
   no integration for it, `integration_id` returns `None` (Agy).
5. Tests that name agents must walk `SUPPORTED`, not a copied list.

Adding a **sidebar field** is the same shape as #76: a saved "everything on"
list will not name the new field. `FieldSet::parse` has to keep reading that
exact legacy list as `all()`, and `as_list` needs a marker for the one new
selection that would collide with it.

## Code Review Rules

For pull-request review, prioritize semantic correctness over whether the happy-path
tests pass. Treat the following as repository-specific invariants and call out
violations explicitly:

1. **Current means current.** Model, context, quota, cache, topic, and session
   fields presented as live/current must come from the newest applicable
   observation. Do not substitute a rollout/file head, an old cache entry, or a
   historical session value just because it is easier to read.
2. **Attribution must be provable.** Never merge or reuse quota, cache, model, or
   session data across accounts, credential scopes, providers, or sessions
   unless the code has evidence that they are the same billing/serving target.
   On ambiguity, prefer missing data over confidently wrong data.
3. **Bounded reads must preserve the requested semantics.** Prefix reads are fine
   for immutable metadata written at the start of a file; latest/current fields
   require a bounded tail or reverse scan, an equivalent seekable snapshot, or
   no value. A performance bound must not silently change "latest" into "first".
4. **Multiple files can be one logical record.** When upstream storage has plain,
   compressed, migrated, temporary, or otherwise alternate representations of
   the same logical object, follow upstream's canonical precedence and make
   transition behavior deterministic. Do not let directory iteration order or
   mtime ties decide correctness.
5. **Pane reads and metadata writes are side effects.** Reject changes that add
   broad pane scans, duplicate publish passes, or no-op metadata writes to
   event/watch/focus paths unless the behavior is explicitly required and
   measured. Preserve the one-pane/event and publish-once invariants above.
6. **Credential stores stay least-privilege.** New code must not broaden secret
   reads, copy credential databases, prompt from background processes, or use a
   less-specific credential scope just to make attribution easier.
7. **Compatibility lists are append-only contracts.** Changes to harnesses,
   providers, sidebar fields, token names, or persisted selection formats must
   preserve ordering/backward-compatibility rules documented in this file and
   include regression coverage for old saved state.
8. **Test representation transitions and stale-data traps.** For storage/cache
   changes, cover both representations when applicable, ambiguous/equal
   timestamps, corrupt or partial data, reused sessions/worktrees, and cases
   where an older value exists but must not be reported as current.

A review should distinguish blocking correctness/attribution/privacy regressions
from non-blocking cleanup or performance suggestions. Green CI is evidence, not
a substitute for checking these invariants.

## Verifying

Run these commands from `useful_tools/herdr_usage/`. Repository automation lives
in `../.github/`; this component keeps its own manifest, lockfile, and toolchain.

```
python3 scripts/security_audit.py
cargo fmt
cargo test
cargo clippy --release
```

Reloading the plugin after a rebuild:

```
herdr plugin disable herdr-agent-usage && herdr plugin enable herdr-agent-usage
```
