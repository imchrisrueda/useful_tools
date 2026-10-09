# Agent setup playbook

This playbook uses Unix shell commands for Linux/macOS or Herdr running inside WSL.
For native Windows use `install.ps1` and the [operational guide](USAGE.es.md);
font mapping still needs verification in the actual terminal.

Instructions for a coding agent installing
[herdr-agent-usage](https://github.com/imchrisrueda/useful_tools) on the
machine that runs Herdr. Humans: paste this file, or the prompt in the README,
into Codex, Agy, or any similar agent **on that machine**.

---

You are installing **herdr-agent-usage** so Herdr's Agent sidebar shows brand
icons, model, context, and subscription quota for the agent CLIs this computer
actually has.

**Done** means the sidebar works. `./install.sh` exiting 0 is only the first
step. Boxes, `?` icons, missing quota, and skipped Herdr integrations mean you
are not finished.

If this working tree is already the repo, enter its `herdr_usage/` directory. Otherwise:

```sh
git clone https://github.com/imchrisrueda/useful_tools.git
cd useful_tools/herdr_usage
```

Then follow this file from the top. Do not improvise a shorter path.

## Rules

- Do **not** stop after `./install.sh`. Read its output and keep going.
- Do **not** run `herdr pane read` (especially `--source recent` /
  `recent-unwrapped`) to "verify" the sidebar. That rebuilds scrollback and
  repaints the agent TUI. Verify with plugin logs, `herdr integration status`,
  font files, terminal config, and by asking the user what they see.
- Herdr plugin **actions ignore extra environment variables**. Pass choices
  through `./install.sh` flags, not `export HERDR_AGENT_QUOTA_…`.
- Use the toolchain in `rust-toolchain.toml` via rustup. Do not `brew install
  rust` or otherwise replace it.
- Ask the user only when a human has to look or act: reload the terminal,
  restart live agent panes, confirm icons.

## 1. Prerequisites

Put user bin dirs on `PATH` before probing (`~/.local/bin`, `~/.cargo/bin`,
`/opt/homebrew/bin`, `/usr/local/bin`).

| Need | How to check | If missing |
| --- | --- | --- |
| Herdr **0.9.0+** | `herdr --version` | Stop. The user must install Herdr first. |
| rustup + Cargo | `command -v rustup cargo` | Install rustup from https://rustup.rs — not a distro/Homebrew `rust` package. `cargo build` then fetches the pin in `rust-toolchain.toml` (currently 1.95.0). |
| macOS or Linux | `uname -s` | Use this playbook on macOS/Linux or inside WSL; native Windows has a PowerShell installer. |

`herdr plugin link` needs a working Herdr client. If `herdr` errors, fix that
before building.

## 2. Detect which agents to enable

Take the **union** of (a) binaries on `PATH`, (b) well-known config/data dirs,
(c) kinds in `herdr agent list` JSON. Names must be the plugin's `--agent`
tokens (`codex`, `agy`).

| `--agent` name | Binaries | Extra evidence |
| --- | --- | --- |
| `codex` | `codex` | `~/.codex` |
| `agy` | `agy` | `~/.gemini/antigravity-cli` |

Supported set: `codex,agy`.

If nothing matches, install **all** and say so. Prefer the detected subset:
`configure` writes Agy `statusLine` entries only for selected agents, and you
should not touch CLIs the user does not have.

Print the detected list before installing.

## 3. Build, link, configure

From the repo root. If this is an existing git checkout on `main`,
`git pull --ff-only` first (stop on divergence; do not force-push or reset
user work).

```sh
./install.sh --agent codex,agy   # detected names, comma-separated
```

`install.sh` builds the release binary, `herdr plugin link --enabled`, writes
prefs into the plugin config dir, then waits for the **configure** action.
That action is what installs sidebar rows, the icon font, and Agy statusLine hooks.

**Read the whole script output.** Lines about `font:` or missing integrations
are remaining work, not noise.

Repair of an existing install is the same command. Do not delete caches or
kill watchers.

## 4. Finish what the script cannot

### Plugin is actually enabled

```sh
herdr plugin list
herdr plugin log list --plugin herdr-agent-usage --limit 20
```

`herdr-agent-usage` must be present and enabled. A failed configure/startup
log is a blocker: read it, fix it, re-run `./install.sh` or

```sh
herdr plugin action invoke configure --plugin herdr-agent-usage
```

and wait until `herdr plugin log list` shows **succeeded** (invoke returns
while the action is still `running`).

### Herdr integrations

Quota attribution for Codex uses Herdr's session integration. Agy does not.

```sh
herdr integration status
```

If Codex is detected and `not installed`, run:

```sh
herdr integration install codex
```

Tell the user to **restart already-running panes** of any integration you
just installed.

### Icon font (boxes, tofu, or `?`)

`configure` copies **Herdr Agent Icons Max** into `~/Library/Fonts` (macOS)
or `~/.local/share/fonts` (Linux) and, if a Ghostty or kitty config **already
exists**, writes a marked `U+E1A0–U+E1B6` / `U+E1C0–U+E1C5` map. It does
**not** create a terminal config from scratch, and it does not map WezTerm,
iTerm, Alacritty, Terminal.app, Warp, or VS Code integrated terminal.

Private Use Area glyphs do not fall back like ordinary missing characters.
Without an explicit map, the cell is a box, blank, or `?`.

1. Confirm the file exists (`HerdrAgentIconsMax-*.ttf` in the font dir).
2. On Linux, run `fc-cache -f "${XDG_DATA_HOME:-$HOME/.local/share}/fonts"`.
3. Detect **this** terminal (`TERM_PROGRAM`, `KITTY_WINDOW_ID`, `WEZTERM_EXECUTABLE`, `TERM`).
4. If Ghostty/kitty maps were written, tell the user to reload (Ghostty:
   `cmd+shift+,`; kitty: `ctrl+shift+f5`) or reopen the terminal.
5. If this terminal has a config but no map, add one (keep the plugin
   markers on Ghostty/kitty so uninstall can remove them):

Ghostty:

```
# BEGIN herdr-agent-usage font
font-codepoint-map = U+E1A0-U+E1B6="Herdr Agent Icons Max"
font-codepoint-map = U+E1C0-U+E1C5="Herdr Agent Icons Max"
# END herdr-agent-usage font
```

Typical paths: `~/Library/Application Support/com.mitchellh.ghostty/config`,
`~/.config/ghostty/config`.

kitty (`~/.config/kitty/kitty.conf`):

```
# BEGIN herdr-agent-usage font
symbol_map U+E1A0-U+E1B6 Herdr Agent Icons Max
symbol_map U+E1C0-U+E1C5 Herdr Agent Icons Max
# END herdr-agent-usage font
```

WezTerm: add `{ family = "Herdr Agent Icons Max" }` to an existing
`font_with_fallback` list; do not replace the user's primary font.

VS Code integrated terminal: append `'Herdr Agent Icons Max'` to
`terminal.integrated.fontFamily`.

iTerm2, Terminal.app, Alacritty, Warp: there is no reliable per-range map.
Say so. Installing the font is still required; Ghostty or kitty will render
the marks.

6. A **yellow `?`** on a brand cell after 1.6.1 is almost always "this
   terminal is not using Herdr Agent Icons Max for U+E1A0–U+E1B6".

### Refresh quota

```sh
herdr plugin action invoke refresh --plugin herdr-agent-usage
```

Wait for that log id to succeed. Invoke returning immediately is not
success.

### Sessions that must restart

Hooks and integrations load at session start:

- Agy: send one turn so StatusLine produces an observation.
- Codex: restart the pane if you just installed Herdr's integration.

## 5. When something still looks wrong

| Symptom | What to do |
| --- | --- |
| Plugin missing / sidebar rows missing | `./install.sh` again, then `configure`. Do not hand-edit Herdr `config.toml` quota rows. |
| Integration `not installed` | `herdr integration install codex`, restart that pane. |
| Agy quota empty | One turn in that session. |
| Icons are boxes / `?` | Font + terminal map + reload; see §4. |
| Gauges meters missing | Sidebar narrower than ~24 columns; widen, then `prefix+shift+r`. |
| `gauges` still the old width | `prefix+shift+r`. There is no live resize publish path. |

```sh
herdr plugin action invoke refresh --plugin herdr-agent-usage
herdr plugin action invoke configure --plugin herdr-agent-usage
```

Settings later: `prefix+shift+q`, or
`herdr plugin pane open --plugin herdr-agent-usage --entrypoint settings --focus`.

## 6. Report to the user

- Agents detected vs enabled
- Integrations installed or still missing
- Font path, which terminal you mapped, and the reload key
- Which live panes to restart, and which need one more turn
- Anything still broken, with the check you used

Do not claim the sidebar is correct unless the user confirmed the icons, or
you verified font + map + plugin logs and stated that the glyphs themselves
need a human look.
