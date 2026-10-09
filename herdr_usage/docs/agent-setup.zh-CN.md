# Agent 安装配置话术

本流程使用 Unix shell，适用于 Linux/macOS 或运行在 WSL 内的 Herdr。
Windows 原生安装使用 `install.ps1`，见[操作指南](USAGE.es.md)；
仍须在当前终端验证字体映射。

给**正在这台电脑上跑 Herdr** 的编程助手看的操作说明。把本文件，或 README
里的提示词，贴给 Codex、Agy 等即可。

---

你要在这台电脑上把 **herdr-agent-usage** 装到真正能用：Herdr 的 Agent 侧栏
要显示出品牌图标、模型、上下文，以及这台机器上实际存在的那些 agent CLI 的
订阅额度。

**完成标准是侧栏可用**，不是 `./install.sh` 返回 0。图标变成方框或 `?`、没有
额度、Herdr integration 仍是 `not installed`，都算没装完。

若当前工作区已经是本仓库，先进入 `herdr_usage/` 目录。否则：

```sh
git clone https://github.com/imchrisrueda/useful_tools.git
cd useful_tools/herdr_usage
```

从本文件开头按步骤做，不要自行缩短流程。

## 硬性规则

- **不要**在 `./install.sh` 结束后停手。读完它的输出，继续做后面的收尾。
- **不要**用 `herdr pane read`（尤其 `--source recent` /
  `recent-unwrapped`）去「验收」侧栏。那会重建 scrollback，把正在跑的 agent
  TUI 重绘一遍。请用插件日志、`herdr integration status`、字体文件、终端
  配置来验证，图标好不好看则问用户。
- Herdr 的插件 **action 看不到你 export 的环境变量**。选择项只能通过
  `./install.sh` 的 flag 传进去。
- 使用 `rust-toolchain.toml` 里的工具链（走 rustup）。不要 `brew install
  rust` 或用发行版 Rust 顶替。
- 只有人必须动手时才问用户：重载终端、重启正在跑的 agent 窗格、看一眼图标对不对。

## 1. 前置条件

探测前先把常见 bin 目录放进 `PATH`（`~/.local/bin`、`~/.cargo/bin`、
`/opt/homebrew/bin`、`/usr/local/bin`）。

| 需要 | 怎么查 | 没有时 |
| --- | --- | --- |
| Herdr **0.9.0+** | `herdr --version` | 停下来，让用户先装 Herdr。 |
| rustup + Cargo | `command -v rustup cargo` | 从 https://rustup.rs 装 rustup，不要用 Homebrew/发行版的 `rust` 包。随后 `cargo build` 会按 `rust-toolchain.toml` 拉取钉死的版本（当前 1.95.0）。 |
| macOS 或 Linux | `uname -s` | 本流程用于 macOS、Linux 或 WSL；Windows 原生版使用 PowerShell 安装器。 |

`herdr plugin link` 需要 Herdr 客户端可用。`herdr` 本身报错就先修这个。

## 2. 探测要启用哪些 agent

取三者的**并集**：（a）`PATH` 上的二进制，（b）常见配置/数据目录，（c）
`herdr agent list` JSON 里的 kind。写入 `--agent` 的必须是下表里的名字（`codex`、`agy`）。

| `--agent` 名 | 二进制 | 额外证据 |
| --- | --- | --- |
| `codex` | `codex` | `~/.codex` |
| `agy` | `agy` | `~/.gemini/antigravity-cli` |

当前支持列表：`codex,agy`。

一个都没探测到：安装 **all**，并明确告诉用户。优先只装探测到的子集——
`configure` 会给选中的 Agy 写 statusLine，不要去动用户没有的 CLI。

安装前先打印探测结果。

## 3. 编译、链接、写入配置

在仓库根目录。若这是已有的 `main` 检出，先 `git pull --ff-only`（分叉了就
停下，不要强推或重置用户的改动）。

```sh
./install.sh --agent codex,agy   # 换成探测到的名字，逗号分隔
```

`install.sh` 会编译 release、`herdr plugin link --enabled`、把偏好写进插件
配置目录，然后**等到 configure action 结束**。sidebar 行、图标字体、
statusLine 都是这一步做的。

**读完脚本的全部输出。** `font:`、缺失 integration 这些行是还没做完的工作，不是可以忽略的提示。

已经装过再跑同一条命令就是升级/修复。不要删缓存，也不要去杀 watcher。

## 4. 脚本做不到的收尾

### 确认插件已启用

```sh
herdr plugin list
herdr plugin log list --plugin herdr-agent-usage --limit 20
```

必须能看到已启用的 `herdr-agent-usage`。configure/startup 日志失败就是
阻塞项：读日志，修好后再跑 `./install.sh`，或：

```sh
herdr plugin action invoke configure --plugin herdr-agent-usage
```

等到 `herdr plugin log list` 显示 **succeeded**。`invoke` 立刻返回时状态
仍是 `running`，那不算出成功。

### Herdr integration

Codex 的额度归属依赖 Herdr 的 session integration。Agy 不需要。

```sh
herdr integration status
```

若已探测到 Codex 且状态为 `not installed`，执行：

```sh
herdr integration install codex
```

刚装上的 integration：告诉用户**重启已经在跑的对应窗格**。

### 图标字体（方框、空白、问号）

`configure` 会把 **Herdr Agent Icons Max** 拷到 `~/Library/Fonts`（macOS）
或 `~/.local/share/fonts`（Linux）；若 Ghostty / kitty 的配置文件**已经
存在**，再写入带标记的 `U+E1A0–U+E1B6` / `U+E1C0–U+E1C5` 映射。它**不会**
从零创建终端配置，也不会给 WezTerm、iTerm、Alacritty、Terminal.app、Warp、
VS Code 集成终端写映射。

PUA 字符不会像普通缺字那样自动回退。没有显式映射时，格子就是方框、空白
或 `?`。

1. 确认字体文件在（字体目录里的 `HerdrAgentIconsMax-*.ttf`）。
2. Linux 上执行 `fc-cache -f "${XDG_DATA_HOME:-$HOME/.local/share}/fonts"`。
3. 判断**当前**终端（`TERM_PROGRAM`、`KITTY_WINDOW_ID`、`WEZTERM_EXECUTABLE`、`TERM`）。
4. 若已经写入 Ghostty/kitty 映射，让用户重载配置（Ghostty：`cmd+shift+,`；
   kitty：`ctrl+shift+f5`）或重开终端。
5. 若当前终端有配置文件但没有映射，补上（Ghostty/kitty 请保留插件的标记
   块，卸载时才能删掉）：

Ghostty：

```
# BEGIN herdr-agent-usage font
font-codepoint-map = U+E1A0-U+E1B6="Herdr Agent Icons Max"
font-codepoint-map = U+E1C0-U+E1C5="Herdr Agent Icons Max"
# END herdr-agent-usage font
```

常见路径：`~/Library/Application Support/com.mitchellh.ghostty/config`、
`~/.config/ghostty/config`。

kitty（`~/.config/kitty/kitty.conf`）：

```
# BEGIN herdr-agent-usage font
symbol_map U+E1A0-U+E1B6 Herdr Agent Icons Max
symbol_map U+E1C0-U+E1C5 Herdr Agent Icons Max
# END herdr-agent-usage font
```

WezTerm：把 `{ family = "Herdr Agent Icons Max" }` 加进已有的
`font_with_fallback`，不要换掉用户的主字体。

VS Code 集成终端：在 `terminal.integrated.fontFamily` 末尾加上
`'Herdr Agent Icons Max'`。

iTerm2、Terminal.app、Alacritty、Warp：没有可靠的按码位映射。如实说明。
字体仍要装；Ghostty 或 kitty 才能稳定显示这些图标。

6. 1.6.1 之后品牌格仍是**黄色 `?`**，多半是当前终端没有把
   U+E1A0–U+E1B6 交给 Herdr Agent Icons Max。

### 拉一次额度

```sh
herdr plugin action invoke refresh --plugin herdr-agent-usage
```

等到对应 log id 成功。`invoke` 立刻返回不等于成功。

### 必须重启的会话

Hook 和 integration 只在会话启动时加载：

- Agy：在该会话里发一轮，StatusLine 才会留下观测。
- Codex：刚装上 Herdr integration 时，重启对应窗格。

## 5. 还是不对时

| 现象 | 做什么 |
| --- | --- |
| 插件没有 / 侧栏行缺失 | 再跑 `./install.sh`，然后 configure。不要手改 Herdr `config.toml` 里的额度行。 |
| integration 仍是 `not installed` | `herdr integration install codex`，再重启该窗格。 |
| Agy 没有额度 | 在该会话发一轮。 |
| 图标是方框 / `?` | 字体 + 终端映射 + 重载；见第 4 节。 |
| gauges 没有进度条 | 侧栏大约窄于 24 列是预期；拉宽后 `prefix+shift+r`。 |
| 拉宽后 gauges 仍是旧长度 | `prefix+shift+r`。没有随拖动实时发布的路径。 |

```sh
herdr plugin action invoke refresh --plugin herdr-agent-usage
herdr plugin action invoke configure --plugin herdr-agent-usage
```

之后改设置：`prefix+shift+q`，或
`herdr plugin pane open --plugin herdr-agent-usage --entrypoint settings --focus`。

## 6. 向用户汇报

- 探测到哪些 agent、实际启用了哪些
- 装了或仍缺哪些 integration
- 字体路径、给哪个终端写了映射、如何重载
- 哪些正在跑的窗格要重启，哪些还要再发一轮
- 仍然坏着的地方，以及你用了哪条检查

不要在用户没看过图标、你也只验证了字体/映射/日志的情况下，声称侧栏已经
正确。glyph 本身需要人看一眼。
