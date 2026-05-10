<center>
<img src='/public/image/BlueRandom.png'>

# Blue Random | 蔚蓝点名
</center>

------

## 项目简介 ✨
蔚蓝点名 是一款基于 Tauri + Rust + Vue 3 的随机点名工具，灵感来源于 **《蔚蓝档案(Blue Archive)》** 的 ~~九蓝一金~~ 学生招募。

## 功能特性 🎯
- 🪟 悬浮按钮快速唤起抽取
- 👥 快速抽取人数选择窗口
- ✉️ ~~不那么~~仿蔚蓝档案的抽奖动画
- 📋 快捷名单导入与权重管理
- 🔁 允许/禁止重复抽取开关
- ⚙️ 应用内配置页

## 快速开箱 📦
- 前往 [Github Actions](https://github.com/Yun-Hydrogen/ba_random_electron/actions) 下载 最新构建，或在 [Github Releases](https://github.com/Yun-Hydrogen/ba_random_electron/releases) 获取手正式版。
- 解压后运行可执行文件，在托盘区域找到 Blue Random 的托盘图标并右键，点击 `配置` 打开应用内配置界面。
- 导入学生名单，并调整偏好设置。
- 开始尽情享受~~抽卡~~吧!

## 项目目录结构 📁
- `public/`  静态资源（图片、音效）
- `src/`  前端渲染与页面
- `src-tauri/`  Tauri/Rust 后端、窗口、托盘与打包配置；`src-tauri/src/` 按配置、命令、窗口、音频、抽取、更新、权限等职责拆分
- 构建完成后，在 `src-tauri/target/release/bundle/` 目录中可找到安装包产物

## 配置说明 🧩
配置文件会保存在程序运行目录下的 `config.yml`。可在 Web 配置页中调整并保存，主要配置包括：
- 名单与权重
- 是否允许重复抽取
- 悬浮按钮参数
- 抽取背景音乐与音效
- Windows 管理员置顶增强与开机计划任务

## 配置入口 🌐
托盘菜单可打开应用内配置页。Tauri 版不再启动本地 Web 配置服务，也不再开放 `localhost` 配置端口。

## 贡献 🤝
欢迎提交 Issue 和 PR！
- **Bug 或建议**：请先在 Issue 中描述复现步骤与期望行为
- **新功能**：建议先开 Issue 讨论方向与实现
- **代码提交**：保持风格一致，必要时补充截图/录屏



## 说明 🧠
- **本项目大部分由 AI 生成与改写。作者对 Tauri、Rust 与 Vue 开发并不熟悉，如有不完善之处，欢迎指正与贡献改进。**
- **本项目采用了部分第三方美术资源，项目作者并没有获得授权，如造成侵权请联系删除。**

## 许可证 📄
- 除项目 **/public/** 下的图片和音乐资源，项目使用 **AGPLv3** 许可证。
- 项目 **/public/** 下的图片和音乐资源由各自版权方所有，使用时请注意授权和范围。

## 感谢 💕
- 《蔚蓝档案(Blue Archive)》游戏提供的灵感：
[国服](https://bluearchive-cn.com/)  [国际服](https://bluearchive.nexon.com/home) [日服](https://bluearchive.jp/)
- 音乐 **KARUT** 的 **《Connected Sky》**。
