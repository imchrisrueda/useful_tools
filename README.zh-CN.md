# herdr-agent-usage

在 Herdr Agent 侧栏显示模型、上下文和订阅额度——按 Space 分组，并用品牌图标承载
agent 状态。

[![CI](https://github.com/imchrisrueda/useful_tools/actions/workflows/ci.yml/badge.svg)](https://github.com/imchrisrueda/useful_tools/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[English](README.md)

<img src="docs/screenshots/sidebar-gauges.png" alt="按 Space 分组的 gauges 侧栏" width="320">

Agent 按所属 Space 分组。每一行只用本插件的品牌图标，不再画 Herdr 原生状态圈；
图标颜色跟随 agent——工作中为黄、完成后为青绿；聚焦该 pane，或焦点从它移走后变为墨白。其他未读的绿色 pane 不受影响。
Provider／模型保持墨白色；进度条上的严重程度色仍表示剩余额度。

默认布局是 `gauges`：在每个额度数字旁加一条进度条。进度条长度始终对应旁边打印的数字；
`cx`、`5h`、`7d`、`30d` 都跟随 `quota-percent`。标签列三个字符，内置周期对齐；
服务商自定义的窗口名过长时退回普通数字行，而不是截断进度条。进度条按当前连接的
Herdr endpoint 侧栏宽度定长（已计入缩进和滚动条）。空字段自动折叠，百分比可选择
显示剩余或已用额度。Cache 与 TTL 默认关闭（需要时可在设置里打开）。Codex
在同一个 Space 里每个标签页都还在 Agent 列表里，只把重复的 5h/7d/30d 收到一行上；
宽栏下主行只留图标、厂商名和额度，子行无图标，只显示 model、topic、cx。设置里的
1 行空格仍隔开不同 agent；同一厂商的嵌套子行贴在一起。Agy 仍按窗格各自通过 StatusLine
显示。Agent order 默认按 Space 分组，组内剩余额度最少的优先。
低额度通知默认关闭，直到你设置阈值。
布局、字段和百分比口径都可以在设置面板里改（`prefix+shift+q`）。

## 安装与升级

要求：**Herdr 0.9.0+**、`rust-toolchain.toml` 指定的 Rust 工具链、macOS 或 Linux
（或 Windows + WSL），以及受支持的 agent CLI（Codex 或 Agy）。

```sh
git clone https://github.com/imchrisrueda/useful_tools.git
cd useful_tools
./install.sh
```

Herdr 插件 id 是 `herdr-agent-usage`；此 GitHub 仓库名为 `useful_tools`。`./install.sh` 会接管已有的
`herdr-agent-quota` 配置和状态，即使 Herdr 已经把链接换成新 id 也会从磁盘上的旧目录
搬过去，然后再 unlink 仍在列表里的旧 id。新二进制第一次启动时也会搬 Herdr 注入的那两个
目录。

只启用指定 agent：`./install.sh --agent codex` 或 `./install.sh --agent codex,agy`。
仅在需要加载新安装的 hook 或 Herdr integration 时，才需重启已经运行的 agent 会话。

脚本会编译、链接并跑 `configure`。它不会在每一种终端里把图标映射好，也不会
做完 Herdr integration。要让这台电脑上的编程助手收尾，把 [让 Agent 装完整](#让-agent-装完整) 里的提示词贴给它。

在仓库目录升级：

```sh
git pull --ff-only
./install.sh
```

升级保留已有偏好，修复插件管理的配置，重新读取额度并自动恢复后台更新。
不需要删除缓存或管理 watcher 进程；Herdr 服务端连接变化后，watcher 会自动接管。

## 让 Agent 装完整

## 让 Agent 装完整

`./install.sh` 不是全部工作：品牌图标要在**当前**终端里映射字体，Codex
还要 Herdr integration。把下面这段贴给**跑 Herdr 的这台电脑**上的 Codex、
Agy 或其他编程助手。完整步骤和命令见
[docs/agent-setup.zh-CN.md](docs/agent-setup.zh-CN.md)
（[English](docs/agent-setup.md)）。

```
把 herdr-agent-usage 在这台电脑上配置到真正能用：Herdr 的 Agent 侧栏要显示
品牌图标，以及我实际安装了的那些 agent CLI 的额度。不要在 ./install.sh
结束后停手。图标变成方框或问号都算没装完。

仓库：https://github.com/imchrisrueda/useful_tools
若当前工作区已经是该仓库就直接用；否则 clone 后进入目录，严格按照
docs/agent-setup.zh-CN.md（中文）或 docs/agent-setup.md（英文）执行。
读不到这两个文件时，仍须做完下面全部步骤。

规则：
- 不要 herdr pane read（尤其 --source recent）。那会重绘正在跑的 agent TUI。
- 插件 action 看不到你 export 的环境变量。选择项用 ./install.sh 的 flag 传。
- 用 rustup 装 rust-toolchain.toml 里的工具链，不要 brew install rust。

1. PATH 先加上 ~/.local/bin、~/.cargo/bin、/opt/homebrew/bin、/usr/local/bin。
   需要 herdr 0.9.0+ 和 rustup/cargo。没有 herdr 就停下来告诉我。没有 cargo
   就装 rustup（https://rustup.rs），不要用发行版/Homebrew 的 rust 包。

2. 探测 agent：PATH 上的二进制、常见配置目录、以及 herdr agent list 的并集。
   --agent 名字：codex（codex，~/.codex）、agy（agy，~/.gemini/antigravity-cli）。
   先打印探测结果。一个都没有就装 all，并说明。

3. 在仓库里：若是干净的 main 先 git pull --ff-only，然后
   ./install.sh --agent <探测到的,逗号分隔>
   读完输出。font: 和缺失 integration 都是还没做完的工作。

4. 脚本之后：
   - herdr plugin list 必须能看到已启用的 herdr-agent-usage。
   - 等到 configure/refresh 日志 succeeded（invoke 会在 running 时就返回）。
   - herdr integration status；若 Codex 为 not installed，执行
     herdr integration install codex（agy 不需要）。
   - 字体：configure 会把 Herdr Agent Icons Max 拷到 ~/Library/Fonts（macOS）
     或 ~/.local/share/fonts（Linux），且仅当 Ghostty/kitty 配置已存在时写入
     映射。Linux 对该字体目录跑 fc-cache。判断当前终端（TERM_PROGRAM /
     KITTY_WINDOW_ID / WEZTERM_EXECUTABLE）。PUA U+E1A0–U+E1B6 必须显式映射，
     否则格子是方框或「?」。Ghostty：
     font-codepoint-map = U+E1A0-U+E1B6="Herdr Agent Icons Max"
     （以及 U+E1C0–U+E1C5），包在 # BEGIN/END herdr-agent-usage font 里。
     kitty：symbol_map 同样范围到 Herdr Agent Icons Max。WezTerm：把该 family
     加进 font_with_fallback。VS Code：追加到 terminal.integrated.fontFamily。
     然后重载终端（Ghostty cmd+shift+,，kitty ctrl+shift+f5）。
     1.6.1 之后仍是黄色「?」多半是终端没映射。
   - herdr plugin action invoke refresh --plugin herdr-agent-usage 并等待结束。
   - 告诉我哪些已经在跑的窗格要重启。Agy 要再发一轮 StatusLine 才会有额度。

5. 汇报：探测到 vs 实际启用的 agent、integration、字体路径和映射了哪个终端、
   还要重启什么、还有什么是坏的。没有人看过图标或未经字体映射验证时，
   不要声称图标已经正确。
```

## 设置

按 `prefix+shift+q` 打开；若该快捷键已有其他用途，可运行：

```sh
herdr plugin pane open --plugin herdr-agent-usage --entrypoint settings --focus
```

<img src="docs/screenshots/settings.png" alt="Agent quota 设置" width="760">

| 设置 | 可选项 |
| --- | --- |
| Percentages | 剩余或已用比例；颜色始终表示剩余额度 |
| Layout | `gauges`（默认）在每个额度数字旁加进度条；`packed` 合并相关字段；`stacked` 将字段分行显示 |
| Row gap | Agent 之间保留零行或一行空白 |
| Watch interval | 30 秒–1 小时，默认 60 秒 |
| Fields | 默认开启提供方、主题、模型、上下文、短期／长期／月度额度；cache 与 TTL 可选 |
| Agent order | 按 Space 分组，组内剩余额度最少优先（默认）；或使用 Herdr 自己的排序 |
| Low quota alert | 关闭，或设置 1%–100% 的提醒阈值 |
| Agents | Codex、Agy |

方向键或空格修改，`a` 应用，`q` 关闭。脚本配置选项见 `./install.sh --help`。

## 数据来源与边界

| Agent | 额度来源 | 归属依据 |
| --- | --- | --- |
| Codex | 本地 rollouts JSONL/.zst；5h 和／或 7d | `~/.codex/sessions` 中的本地会话窗口 |
| Agy / Antigravity | StatusLine JSON 载荷；5h、7d，以及 Gemini 会话上的 api（第三方池） | 精确会话与可确认的模型额度池 |

Agy / Antigravity 状态栏捕获本地 StatusLine 输出，并在末尾追加当前生效额度窗口的
消耗节奏，例如 `⏱ 5h ↓12%`：已用额度减去窗口已过去的时间比例，单位为百分点。
`↓` 表示应放慢，`↑` 表示还有余量，`=` 表示相差五个点以内。以剩余额度最少的窗口为
准并标明窗口（`5h`/`7d`）；该窗口无法计算节奏时不显示，也不改用较宽松的窗口。

额度窗口保留上游定义。模型、上下文和缓存数据优先来自已识别的会话。
`ttl≈` 表示估算的提示词缓存寿命，不保证实际过期时间。
主题提取只读取事件点名窗格的可见屏幕；内容滚走后保留已有主题。

所有受支持的工作中 agent 共用一个后台 watcher，请求间隔至少 60 秒，并在回合结束后
完成收尾刷新。零远程 HTTP 网络流量，零磁盘凭据。

Agy 没有可靠的服务账号 ID，因此不跨会话共享观测值。
账号或模型额度池无法确认时不猜测数字。请求失败保留同一账号最后一次已确认的读数，
不会把失败解释为零用量。

## 常见问题

| 现象 | 检查 |
| --- | --- |
| 品牌图标是方框或 `?` | 字体没装上，或当前终端没有 U+E1A0–U+E1B6 映射——见 [让 Agent 装完整](#让-agent-装完整)。`configure` 之后要重载终端。1.6.1 之前工作态的黄色 `?` 是 ZWNJ 的 bug，先升级。 |
| 缺少会话数据 | 运行 `herdr integration status`，安装缺失项后重启对应 agent |
| Agy 缺少额度 | 发送一轮消息，让该会话的 StatusLine 产生观测 |
| 缺少侧栏行 | 运行下面的 configure action 修复插件配置 |
| 侧栏太窄，`gauges` 不显示进度条 | 约 24 列以下是预期行为；调宽后刷新即可 |
| 调整宽度后 `gauges` 仍是旧长度 | 用 `prefix+shift+r` 刷新；没有随拖动实时发布的路径 |
| `gauges` 下 cache 信息仍分两行 | 调宽侧栏，直到合并后的整行放得下 |

```sh
herdr plugin action invoke refresh --plugin herdr-agent-usage
herdr plugin action invoke configure --plugin herdr-agent-usage
```

完整卸载使用 `./uninstall.sh`，只移除部分 agent 使用 `./uninstall.sh --agent codex`。
配置修改可恢复，用户自己的设置与其他 agent 不受影响。

## 参与开发

开发与验证见 [CONTRIBUTING.md](CONTRIBUTING.md)，数据处理及漏洞报告见
[SECURITY.md](SECURITY.md)，版本变更见 [CHANGELOG.md](CHANGELOG.md)。
历史调研索引见 [docs/README.md](docs/README.md)。

## 许可证

[MIT](LICENSE)。本项目与 Herdr 及受支持的 AI 供应商无隶属关系。
