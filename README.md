<p align="center">
  <img src="/public/image/BlueRandom.png" alt="KVRandom" width="160">
</p>

# KVRandom

<p align="center">
  <a href="https://github.com/MF-Dust/KVRandom/actions/workflows/build-windows.yml"><img src="https://img.shields.io/github/actions/workflow/status/MF-Dust/KVRandom/build-windows.yml?branch=main&label=Windows%20build&style=flat-square" alt="Windows build status"></a>
  <a href="https://github.com/MF-Dust/KVRandom/releases"><img src="https://img.shields.io/github/v/release/MF-Dust/KVRandom?style=flat-square" alt="GitHub release"></a>
  <a href="https://github.com/MF-Dust/KVRandom/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue?style=flat-square" alt="License: AGPL-3.0"></a>
  <img src="https://img.shields.io/badge/platform-Windows-0078d4?style=flat-square&logo=windows&logoColor=white" alt="Platform: Windows">
  <img src="https://img.shields.io/badge/Tauri-2-24c8db?style=flat-square&logo=tauri&logoColor=white" alt="Tauri 2">
  <img src="https://img.shields.io/badge/Vue-3-42b883?style=flat-square&logo=vuedotjs&logoColor=white" alt="Vue 3">
  <img src="https://img.shields.io/badge/Rust-backend-b7410e?style=flat-square&logo=rust&logoColor=white" alt="Rust backend">
  <a href="https://deepwiki.com/MF-Dust/KVRandom"><img src="https://img.shields.io/badge/DeepWiki-open-blue?style=flat-square" alt="Open in DeepWiki"></a>
  <a href="https://zread.ai/MF-Dust/KVRandom"><img src="https://img.shields.io/badge/Zread-open-6f42c1?style=flat-square" alt="Open in Zread"></a>
</p>

KVRandom 是一款基于 Tauri 2、Rust 和 Vue 3 的 Windows 桌面随机点名工具。应用提供悬浮按钮、抽取人数选择、名单权重管理、结果动画、音乐音效、托盘配置页和 Windows 置顶增强等功能。

项目灵感来自《蔚蓝档案(Blue Archive)》的学生招募表现形式，但本项目不是官方项目，也不隶属于相关权利方。

## 功能特性

- 悬浮按钮：常驻桌面，点击后打开抽取人数窗口。
- 抽取人数：支持 1-10 人，可设置默认人数和背景暗度。
- 名单管理：支持文本粘贴、TXT/CSV 导入、去重、权重保留和权重重置。
- 抽取策略：支持允许重复抽取和不重复抽取两种模式。
- 权重抽取：重复抽取使用权重池缓存，不重复抽取按权重进行无放回抽样。
- 结果动画：使用蓝、金、粉信封样式展示结果，并带保底表现。
- 音频反馈：支持按钮点击音、抽取背景音乐和结果动画音效。
- 配置页面：通过托盘菜单打开，无需本地 Web 服务端口。
- Windows 增强：支持管理员置顶增强和管理员权限开机计划任务。
- 更新检查：从 GitHub Releases 读取 `version.yml` 判断是否有新版本。

## 代码阅读与文档入口

除了 GitHub 源码，项目也提供适合快速阅读和问答的外部入口：

- DeepWiki: <https://deepwiki.com/MF-Dust/KVRandom>
  适合浏览由仓库生成的结构化代码说明、模块关系和实现概览。
- Zread: <https://zread.ai/MF-Dust/KVRandom>
  适合用 AI 代码阅读方式查看仓库文档、源码结构和项目摘要。
- Shields.io: <https://shields.io/>
  README 顶部徽章使用 Shields 风格展示构建状态、发布版本、技术栈和文档入口。

## 下载与使用

发布包在 GitHub Releases 中提供：

- Releases: <https://github.com/MF-Dust/KVRandom/releases>
- Actions: <https://github.com/MF-Dust/KVRandom/actions>

常见产物：

- `KVRandom_*_x64-setup.exe`：NSIS 安装包。
- `KVRandom_*_x64_en-US.msi`：MSI 安装包。
- `kvrandom-windows-portable.zip`：便携版，包含 `kvrandom.exe`、`kvrandom_lib.dll` 和 `_up_` 资源目录。

基本使用：

1. 安装或解压发布包。
2. 运行 `kvrandom.exe`。
3. 在系统托盘中右键 KVRandom 图标，点击 `配置`。
4. 导入或粘贴学生名单，调整权重和抽取偏好。
5. 点击桌面悬浮按钮开始抽取。

便携版不要只移动 `kvrandom.exe`，需要保留同目录下的 `kvrandom_lib.dll` 和 `_up_` 资源目录，否则图片、音乐或音效可能无法加载。

## 配置说明

应用会在运行目录下读写 `config.yml`。如果文件不存在，程序会生成默认配置；如果字段缺失或超出范围，程序会在加载时归一化并写回。

主要配置项：

- `studentList`：抽取名单，每项包含 `name` 和 `weight`。
- `allowRepeatDraw`：是否允许重复抽取。
- `floatingButton.sizePercent`：悬浮按钮大小百分比，默认 `100`。
- `floatingButton.transparencyPercent`：悬浮按钮透明度，默认 `20`。
- `floatingButton.alwaysOnTop`：悬浮按钮是否置顶。
- `floatingButton.position`：悬浮按钮屏幕坐标，退出或拖动结束时自动保存。
- `pickCountDialog.defaultPlayMusic`：人数选择窗口是否默认播放背景音乐。
- `pickCountDialog.backgroundDarknessPercent`：抽取遮罩背景暗度。
- `pickCountDialog.defaultCount`：默认抽取人数，范围 `1-10`。
- `pickResultDialog.defaultPlayGachaSound`：结果动画是否默认播放音效。
- `pickResultDialog.gachaSoundVolume`：结果动画音效音量，范围 `0.0-1.0`。
- `webConfig.adminTopmostEnabled`：Windows 下启动时是否尝试管理员置顶增强。
- `webConfig.adminAutoStartEnabled`、`adminAutoStartPath`、`adminAutoStartTaskName`：管理员权限计划任务相关配置。

`webConfig.port` 是兼容旧版本的保留字段。当前 Tauri 版不启动本地 Web 配置服务，也不开放 `localhost` 配置端口。

## 本地开发

环境要求：

- Node.js 20+
- Rust stable
- Windows 环境用于完整桌面功能和安装包构建

常用命令：

```powershell
npm ci
npm run dev
npm run dev:frontend
npm run build:frontend
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
```

说明：

- `npm run dev` 启动完整 Tauri 开发应用。
- `npm run dev:frontend` 只启动 Vite 前端。
- `npm run build:frontend` 只构建 renderer。
- `npm run build` 构建 Windows Tauri 安装包和 release 可执行文件。
- Rust 代码格式化使用 `cargo fmt --manifest-path src-tauri/Cargo.toml`。

## 项目结构

```text
.
├─ public/
│  ├─ image/          # 图片、图标和信封素材
│  └─ sound/          # 点击音、背景音乐、抽取音效
├─ src/
│  ├─ api/            # Tauri command/event 封装
│  ├─ components/     # Vue 组件
│  ├─ composables/    # 配置、日志、弹窗、更新等组合逻辑
│  ├─ router/         # 前端路由
│  └─ views/          # 悬浮窗、配置页、人数选择、结果页
├─ src-tauri/
│  ├─ capabilities/   # Tauri 权限配置
│  ├─ src/            # Rust 后端逻辑
│  ├─ Cargo.toml
│  └─ tauri.conf.json
├─ .github/workflows/ # Windows 构建和发布工作流
├─ config.yml         # 示例/本地运行配置
└─ package.json
```

Rust 后端按职责拆分：

- `commands.rs`：Tauri commands。
- `config.rs`：配置结构、默认值、归一化、名单解析和配置持久化。
- `picker.rs`：权重抽取逻辑。
- `windows.rs`：Tauri 窗口创建、显示、隐藏和位置保存。
- `audio.rs`：基于 rodio 的音频播放线程。
- `tray.rs`：系统托盘菜单。
- `admin.rs`：Windows 管理员权限、单实例锁和计划任务。
- `update.rs`：GitHub Releases 更新检查。

## 发布流程

GitHub Actions 工作流位于 `.github/workflows/build-windows.yml`。

触发方式：

- 手动运行 `workflow_dispatch`，填写 release tag 和 release name。
- 向 `main` 或 `master` 推送包含 `[ci]` 的提交信息。
- 提交信息包含 `[unstable]` 时发布为 preview prerelease。

工作流会：

1. 安装 Node 和 Rust。
2. 执行 `npm ci`。
3. 执行 `npm run build`。
4. 复制 NSIS 和 MSI 安装包。
5. 生成 `version.yml`，用于应用内更新检查。
6. 生成 `kvrandom-windows-portable.zip`，包含 exe、DLL 和 `_up_` 资源目录。
7. 上传 artifact 并发布到 GitHub Releases。

## 资源与音频

Tauri 会把 `public/` 打包为外部资源。安装版和便携版运行时会从以下位置查找资源：

- `public/...`
- `_up_/public/...`
- 兼容性的直接相对路径

如果发布版没有图片或没有声音，优先检查运行目录是否保留了 `_up_/public/image` 和 `_up_/public/sound`。

## 贡献

欢迎提交 Issue 和 PR。

- Bug 报告请包含复现步骤、实际表现、期望表现和运行方式。
- UI 改动建议附截图或录屏。
- 发布和打包相关改动请说明是否验证过 `npm run build`。
- 提交信息建议沿用现有中文前缀，例如 `功能:`、`修复:`、`优化:`、`项目:`、`版本号:`、`Agent:`。

## 说明

- 本项目大部分由 AI 生成与改写，欢迎继续审查和改进。
- 本项目包含部分第三方美术和音乐资源，项目作者并未获得完整授权。如造成侵权，请联系删除或替换。
- 项目内 `public/` 资源的版权归各自权利方所有，使用时请自行确认授权范围。

## 许可证

- 除 `public/` 下的图片和音乐资源外，项目代码使用 AGPLv3 许可证。
- `public/` 下的图片和音乐资源不随项目代码许可证授权。

## 商标与版权声明

“蔚蓝档案”是上海星啸网络科技有限公司的注册商标，版权所有。

「ブルーアーカイブ」は株式会社Yostarの登録商標です。著作権はすべて保有されています。

"Blue Archive" is a registered trademark of NEXON Korea Corp. & NEXON GAMES Co., Ltd. All rights reserved.

## 感谢

- 《蔚蓝档案(Blue Archive)》提供的视觉与交互灵感：
  [国服](https://bluearchive-cn.com/) / [国际服](https://bluearchive.nexon.com/home) / [日服](https://bluearchive.jp/)
- 音乐 KARUT 的《Connected Sky》。
