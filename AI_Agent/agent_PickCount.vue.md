# PickCount.vue 维护说明

本文总结 [src/renderer/views/PickCount.vue](src/renderer/views/PickCount.vue) 的结构与方法，便于后续 AI 维护。

## 模块概览
- 作用：抽取人数选择弹层（带背景暗度、音乐开关、点击音效）。
- 技术：Vue 3 `<script setup>`，通过 `window.pickCountApi` 与主进程交互。
- 关键行为：窗口打开时读取配置；点击确认/取消触发退出动画并通知主进程；BGM 仅在窗口打开后按需播放。

## 页面结构（Template）
- 遮罩层：`.pick-overlay`，根据 `backgroundDarknessPercent` 生成透明黑背景。
- 面板：`.pick-panel`，标题、人数选择、动作按钮、音乐开关。
- 人数选择：`- / +` 按钮 + 数字显示框。
- 快捷设置：最小 / 最大按钮 + 范围提示。
- 操作按钮：取消 / 确定。
- 音乐开关：勾选后播放背景音乐。

## 关键状态（Refs / Computed）
- `count`：当前人数（1-10）。
- `playMusic`：是否播放 BGM。
- `isLeaving`：退出动画中，禁止交互。
- `backgroundDarknessPercent`：遮罩黑度百分比。
- `isDialogOpen`：窗口是否处于打开状态，用于避免误触发播放。
- `isInitializing`：初始化配置中，避免 watch 触发播放。
- `canDecrease`：`count > 1`。
- `canIncrease`：`count < 10`。
- `overlayStyle`：根据暗度生成背景颜色。

## 主要方法与职责
- `resolveAssetUrl(relativePath)`：兼容 `file://` 与 `http://` 的资源路径解析。
- `clampInt(value, min, max, fallback)`：整数范围钳制。
- `initConfig()`：通过 `pickCountApi.getConfig()` 初始化人数、背景暗度、默认播放音乐。
- `resetDialogStateFromConfig(shouldPlayBgm)`：重置状态 + 按需播放 BGM。
- `increaseCount()` / `decreaseCount()`：人数增减并播放点击音效。
- `setMinCount()` / `setMaxCount()`：一键设置最小/最大人数。
- `playClickSound()`：播放按钮点击音效。
- `playBgm()` / `stopAudio()`：播放/停止背景音乐。
- `beginExit(action)`：触发退出动画，延迟通知主进程 `confirm/cancel`。
- `handleConfirm()` / `handleCancel()`：调用 `beginExit()`。

## 监听与副作用
- `watch(playMusic)`：
  - 仅在 `isDialogOpen` 且非 `isInitializing` 时触发。
  - `true` 播放 BGM，`false` 停止。
- `pickCountApi.onOpen`：窗口打开时重置并播放 BGM（若配置开启）。
- `pickCountApi.onStopBgm`：外部通知时停止 BGM。

## 生命周期
- `onMounted()`：
  - 停止音频、读取配置。
  - 绑定 `onOpen` 与 `onStopBgm` 监听。
- `onBeforeUnmount()`：
  - 停止音频、解除监听。

## IPC / API 依赖
来自 `window.pickCountApi`：
- `getConfig()`：读取窗口配置。
- `confirm(count, playMusic)`：确认人数并告知是否播放音乐。
- `cancel()`：取消。
- `onOpen(callback)`：窗口打开时回调。
- `onStopBgm(callback)`：停止 BGM 指令。

## 资源依赖
- 背景音乐：`/sound/bgm.mp3`
- 点击音效：`/sound/button_click.wav`

## 动画与时序
- 遮罩淡入淡出：`pick-overlay-fade-in` / `pick-overlay-fade-out`。
- 面板飞入/飞出：`pick-panel-fly-fade-in` / `pick-panel-fly-fade-out`。
- 退出时延：`EXIT_ANIMATION_MS = 400`，保证动画完成后再通知主进程。

## 维护注意事项
- 若修改人数范围，需同步更新 `clampInt`、`canIncrease`、`min/max` 属性与 UI 文案。
- 默认播放音乐仅在 `onOpen` 阶段触发，避免应用启动即播放。
- 音频对象为单例复用，修改音量或更换资源时注意重置。
