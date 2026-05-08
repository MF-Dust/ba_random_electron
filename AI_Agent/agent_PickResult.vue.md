# PickResult.vue 维护说明

本文总结 [src/renderer/views/PickResult.vue](src/renderer/views/PickResult.vue) 的结构与方法，便于后续 AI 维护。

## 模块概览
- 作用：抽取结果展示页，包含信件飞入动画 + 姓名依次展开 + 抽取音效。
- 技术：Vue 3 `<script setup>`，渲染端通过 `window.pickResultApi` 获取结果与配置。
- 行为：点击任意位置或按 `Esc` 关闭结果页。

## 页面结构（Template）
- 容器：`.result-stage` 全屏遮罩，负责点击关闭。
- 结果行：`.result-rows` 可单排或双排。
  - `topRow`：最多 5 个结果。
  - `bottomRow`：第 6 个及之后结果。
- 卡片：`.letter-card` 信封图片，延迟入场动画。
- 名字卡：`.name-card` 依次展开显示。
- 提示文案：
  - 有结果：`.result-hint`。
  - 无结果：`.result-empty`。

## 关键状态（Refs / Computed）
- `results`：当前抽取结果数组（对象含 `name`）。
- `animationKey`：用于重置动画的 key。
- `instructionText`：底部提示文案。
- `revealStarted`：是否开始展开姓名。
- `canClose`：动画结束后才允许关闭。
- `isClosing`：是否正在执行淡出关闭。
- `lastToken`：结果批次序号，用于忽略过期 reset 事件。
- `playGachaSound`：是否播放抽取音效。
- `gachaSoundVolume`：音效音量。
- `topRow` / `bottomRow` / `isTwoRows`：拆分结果并判断是否双排。

## 主要方法与职责
- `resolveAssetUrl(relativePath)`：兼容 `file://` 与 `http://` 的资源路径解析。
- `normalizeResults(payload)`：兼容多种结果格式，统一为 `{ name }[]`。
- `applyResults(payload)`：
  - 设置结果与动画 key。
  - 写入 `payload.token`，标记当前批次。
  - 重置展开状态与计时器。
  - 根据结果数量计算展开延迟（`(n-1)*120 + 600` ms）。
  - 关闭只在动画结束后允许（总时长约 `+450` ms）。
  - 触发抽取音效播放。
- `closeResult()`：
  - 清空结果、停止音效、清理计时器。
  - 先触发淡出动画（约 220ms），再等待一次渲染帧后通知主进程关闭。
  - 通过 `window.pickResultApi.close()` 通知主进程关闭。
- `handleStageClick()` / `handleKeydown()`：仅在 `canClose` 为 true 时允许关闭。
- `handleReset(payload)`：根据 `token` 与 `reason` 忽略过期 reset，避免清空刚打开的结果。
- `playGachaLoadingSound()`：创建/复用 `Audio` 播放音效，设置音量。
- `stopGachaLoadingSound()`：暂停并归零音效。
- `loadSoundConfig()`：读取 `window.pickResultApi.getConfig()` 获取 `defaultPlayGachaSound` 与音量。

## 生命周期
- `onMounted()`：
  - 加载音效配置。
  - 读取初始结果 `getResults()` 并渲染。
  - 监听 `pick-result:open` 事件，更新配置并渲染新结果。
  - 监听 `pick-result:reset` 事件，清空结果并重置动画状态。
- `onBeforeUnmount()`：
  - 清理计时器与音效。
  - 移除 `onOpen` 监听。
  - 移除 `onReset` 监听。

## IPC / API 依赖
来自 `window.pickResultApi`：
- `getResults()`：获取初始结果。
- `getConfig()`：获取音效配置。
- `onOpen(callback)`：监听结果窗口打开事件。
- `onReset(callback)`：监听结果窗口重置事件。
- `close()`：关闭结果窗口。

## 动画与时序
- 信封飞入：`.letter-card` 使用 `letter-fly-in` 动画，延迟 `index * 0.12s`。
- 姓名展开：`.name-card.is-reveal` 使用 `name-reveal` 动画，延迟 `index * 0.12s + 0.1s`。
- 展开启动：`applyResults()` 根据结果数量计算总延迟后设置 `revealStarted = true`。

## 资源依赖
- 信封图片：`/image/letter.png`
- 抽取音效：`/sound/gacha_loading.wav`

## 样式要点
- `.result-stage` 作为遮罩层，背景为半透明黑。
- `.letter-card` 初始 `scale(2.5)` + `rotate(15deg)`，落位后保持 15 度倾斜。
- `.name-card` 初始透明 + 下移，触发 `is-reveal` 后展开。

## 维护注意事项
- 若修改结果数量阈值（当前 5 个一行），需同步调整 `topRow` / `bottomRow` 与 `--index` 计算。
- 音效配置来自主进程，新增字段时需同步更新 `loadSoundConfig()`。
- 关闭逻辑集中在 `closeResult()`，若新增交互请调用该方法以保证清理完整。
