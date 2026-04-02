const { app, BrowserWindow, Menu, Tray, nativeImage, ipcMain } = require('electron');
const fs = require('fs');
const path = require('path');
const yaml = require('js-yaml');

const DEFAULT_CONFIG = {
  floatingButton: {
    sizePercent: 100,
    transparencyPercent: 20,
    alwaysOnTop: true,
    position: {
      x: null,
      y: null
    }
  },
  pickCountDialog: {
    defaultPlayMusic: false,
    backgroundDarknessPercent: 50,
    backgroundBlurPercent: 10,
    defaultCount: 1
  }
};

let currentConfig = DEFAULT_CONFIG;
const dragSessions = new Map();
let appTray = null;
let floatingButtonWindow = null;
let pickCountWindow = null;
let isQuitting = false;
const FLOATING_WINDOW_FADE_MS = 400;

function clampNumber(value, min, max, fallback) {
  const num = Number(value);
  if (Number.isNaN(num)) return fallback;
  return Math.min(max, Math.max(min, num));
}

function normalizeConfig(input) {
  const source = input && typeof input === 'object' ? input : {};
  const fb = source.floatingButton && typeof source.floatingButton === 'object' ? source.floatingButton : {};
  const position = fb.position && typeof fb.position === 'object' ? fb.position : {};
  const pick = source.pickCountDialog && typeof source.pickCountDialog === 'object' ? source.pickCountDialog : {};

  const alwaysOnTop =
    typeof fb.alwaysOnTop === 'boolean' ? fb.alwaysOnTop : DEFAULT_CONFIG.floatingButton.alwaysOnTop;

  return {
    floatingButton: {
      sizePercent: clampNumber(
        fb.sizePercent,
        0,
        1000,
        DEFAULT_CONFIG.floatingButton.sizePercent
      ),
      transparencyPercent: clampNumber(
        fb.transparencyPercent,
        0,
        100,
        DEFAULT_CONFIG.floatingButton.transparencyPercent
      ),
      alwaysOnTop,
      position: {
        x: Number.isFinite(Number(position.x)) ? Math.round(Number(position.x)) : null,
        y: Number.isFinite(Number(position.y)) ? Math.round(Number(position.y)) : null
      }
    },
    pickCountDialog: {
      defaultPlayMusic:
        typeof pick.defaultPlayMusic === 'boolean' ? pick.defaultPlayMusic : DEFAULT_CONFIG.pickCountDialog.defaultPlayMusic,
      backgroundDarknessPercent: clampNumber(
        pick.backgroundDarknessPercent,
        0,
        100,
        DEFAULT_CONFIG.pickCountDialog.backgroundDarknessPercent
      ),
      backgroundBlurPercent: clampNumber(
        pick.backgroundBlurPercent,
        0,
        100,
        DEFAULT_CONFIG.pickCountDialog.backgroundBlurPercent
      ),
      defaultCount: Math.round(
        clampNumber(
          pick.defaultCount,
          1,
          10,
          DEFAULT_CONFIG.pickCountDialog.defaultCount
        )
      )
    }
  };
}

function getConfigPath() {
  return path.join(process.cwd(), 'config.yml');
}

function toConfigYamlWithComments(config) {
  const fb = config.floatingButton;
  const pick = config.pickCountDialog;
  const posX = Number.isFinite(Number(fb.position.x)) ? String(Math.round(Number(fb.position.x))) : 'null';
  const posY = Number.isFinite(Number(fb.position.y)) ? String(Math.round(Number(fb.position.y))) : 'null';

  return [
    '# 悬浮按钮配置',
    'floatingButton:',
    '  # 按钮大小百分比（基准 50px*50px），范围 0-1000，默认 100',
    `  sizePercent: ${fb.sizePercent}`,
    '  # 透明度百分比，范围 0-100（0=完全不透明，100=完全透明），默认 20',
    `  transparencyPercent: ${fb.transparencyPercent}`,
    '  # 是否置顶（true/false），默认 true',
    `  alwaysOnTop: ${fb.alwaysOnTop ? 'true' : 'false'}`,
    '  # 悬浮按钮窗口位置（左上角屏幕坐标），退出时自动保存；null 表示使用系统默认位置',
    '  position:',
    `    x: ${posX}`,
    `    y: ${posY}`,
    '',
    '# 人数选择窗口配置',
    'pickCountDialog:',
    '  # 是否默认播放喜庆点名音乐（true/false），默认 false',
    `  defaultPlayMusic: ${pick.defaultPlayMusic ? 'true' : 'false'}`,
    '  # 背景变暗程度，范围 0-100（100 接近全黑），默认 50',
    `  backgroundDarknessPercent: ${pick.backgroundDarknessPercent}`,
    '  # 背景模糊程度，范围 0-100，默认 10',
    `  backgroundBlurPercent: ${pick.backgroundBlurPercent}`,
    '  # 人数默认值，范围 1-10 的整数，默认 1',
    `  defaultCount: ${pick.defaultCount}`,
    ''
  ].join('\n');
}

function saveConfig(config) {
  const configPath = getConfigPath();
  const yamlText = toConfigYamlWithComments(config);
  fs.writeFileSync(configPath, yamlText, 'utf8');
}

function writeDefaultConfigIfMissing(configPath) {
  if (fs.existsSync(configPath)) {
    return;
  }
  saveConfig(DEFAULT_CONFIG);
}

function loadConfig() {
  const configPath = getConfigPath();
  writeDefaultConfigIfMissing(configPath);

  try {
    const raw = fs.readFileSync(configPath, 'utf8');
    const parsed = yaml.load(raw);
    const normalized = normalizeConfig(parsed);
    saveConfig(normalized);
    return normalized;
  } catch (error) {
    console.error('Failed to load config.yml, using defaults.', error);
    const fallback = normalizeConfig(DEFAULT_CONFIG);
    saveConfig(fallback);
    return fallback;
  }
}

function persistFloatingButtonPosition() {
  if (!floatingButtonWindow || floatingButtonWindow.isDestroyed()) {
    return;
  }

  const bounds = floatingButtonWindow.getBounds();
  currentConfig = normalizeConfig({
    floatingButton: {
      ...currentConfig.floatingButton,
      position: {
        x: bounds.x,
        y: bounds.y
      }
    }
  });
  saveConfig(currentConfig);
}

function animateWindowOpacity(win, fromOpacity, toOpacity, durationMs) {
  return new Promise((resolve) => {
    if (!win || win.isDestroyed()) {
      resolve();
      return;
    }

    const start = Date.now();
    const delta = toOpacity - fromOpacity;
    win.setOpacity(fromOpacity);

    const timer = setInterval(() => {
      if (!win || win.isDestroyed()) {
        clearInterval(timer);
        resolve();
        return;
      }

      const elapsed = Date.now() - start;
      const t = Math.min(1, elapsed / durationMs);
      win.setOpacity(fromOpacity + delta * t);

      if (t >= 1) {
        clearInterval(timer);
        resolve();
      }
    }, 16);
  });
}

async function fadeOutFloatingButtonWindow() {
  if (!floatingButtonWindow || floatingButtonWindow.isDestroyed()) {
    return;
  }

  if (!floatingButtonWindow.isVisible()) {
    return;
  }

  const currentOpacity = floatingButtonWindow.getOpacity();
  await animateWindowOpacity(
    floatingButtonWindow,
    Number.isFinite(currentOpacity) ? currentOpacity : 1,
    0,
    FLOATING_WINDOW_FADE_MS
  );

  if (floatingButtonWindow && !floatingButtonWindow.isDestroyed()) {
    floatingButtonWindow.hide();
    floatingButtonWindow.setOpacity(1);
  }
}

async function fadeInFloatingButtonWindow() {
  if (!floatingButtonWindow || floatingButtonWindow.isDestroyed()) {
    return;
  }

  floatingButtonWindow.setOpacity(0);
  floatingButtonWindow.show();
  floatingButtonWindow.focus();

  await animateWindowOpacity(floatingButtonWindow, 0, 1, FLOATING_WINDOW_FADE_MS);
}

function createFloatingButtonWindow() {
  currentConfig = loadConfig();
  const config = currentConfig;
  const sizePx = Math.round(50 * (config.floatingButton.sizePercent / 100));

  const winWidth = Math.max(72, sizePx + 20);
  const winHeight = Math.max(72, sizePx + 20);

  const hasSavedX = Number.isFinite(Number(config.floatingButton.position.x));
  const hasSavedY = Number.isFinite(Number(config.floatingButton.position.y));

  const windowOptions = {
    width: winWidth,
    height: winHeight,
    frame: false,
    resizable: false,
    minimizable: false,
    maximizable: false,
    hasShadow: true,
    transparent: true,
    alwaysOnTop: config.floatingButton.alwaysOnTop,
    skipTaskbar: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  };

  if (hasSavedX && hasSavedY) {
    windowOptions.x = Math.round(Number(config.floatingButton.position.x));
    windowOptions.y = Math.round(Number(config.floatingButton.position.y));
  }

  const win = new BrowserWindow(windowOptions);
  floatingButtonWindow = win;

  win.setMenuBarVisibility(false);
  win.loadFile(path.join(__dirname, 'renderer', 'index.html'));

  win.on('closed', () => {
    floatingButtonWindow = null;
  });
}

function closePickCountWindow() {
  if (!pickCountWindow || pickCountWindow.isDestroyed()) {
    fadeInFloatingButtonWindow();
    return;
  }

  const win = pickCountWindow;
  pickCountWindow = null;
  win.close();
}

function createPickCountWindow() {
  if (pickCountWindow && !pickCountWindow.isDestroyed()) {
    pickCountWindow.focus();
    return;
  }

  const win = new BrowserWindow({
    frame: false,
    transparent: true,
    fullscreen: true,
    resizable: false,
    minimizable: false,
    maximizable: false,
    movable: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  });

  pickCountWindow = win;
  win.setMenuBarVisibility(false);
  win.loadFile(path.join(__dirname, 'renderer', 'pick-count.html'));

  win.on('closed', () => {
    pickCountWindow = null;
    fadeInFloatingButtonWindow();
  });

  fadeOutFloatingButtonWindow();
}

function createTray() {
  const trayIconPath = path.join(__dirname, 'image', 'tray.png');
  const trayIcon = nativeImage.createFromPath(trayIconPath);
  appTray = new Tray(trayIcon);

  appTray.setToolTip('BA Random Electron');
  appTray.setContextMenu(
    Menu.buildFromTemplate([
      {
        label: '退出',
        click: () => {
          app.quit();
        }
      }
    ])
  );
}

ipcMain.handle('floating-button:get-config', () => {
  return currentConfig.floatingButton;
});

ipcMain.on('floating-button:clicked', () => {
  createPickCountWindow();
});

ipcMain.handle('pick-count:get-config', () => {
  return currentConfig.pickCountDialog;
});

ipcMain.on('pick-count:cancel', () => {
  closePickCountWindow();
});

ipcMain.on('pick-count:confirm', (event, payload) => {
  const selectedCount = Math.round(clampNumber(payload && payload.count, 1, 10, 1));
  const playMusic = Boolean(payload && payload.playMusic);
  console.log(`Pick count confirmed. count=${selectedCount}, playMusic=${playMusic}`);
  closePickCountWindow();
});

ipcMain.on('floating-button:drag-start', (event, payload) => {
  const win = BrowserWindow.fromWebContents(event.sender);
  if (!win) return;

  const bounds = win.getBounds();
  dragSessions.set(event.sender.id, {
    startWinX: bounds.x,
    startWinY: bounds.y
  });
});

ipcMain.on('floating-button:drag-move', (event, payload) => {
  const win = BrowserWindow.fromWebContents(event.sender);
  const session = dragSessions.get(event.sender.id);
  if (!win || !session || !payload) return;

  const dx = Number(payload.dx);
  const dy = Number(payload.dy);
  if (Number.isNaN(dx) || Number.isNaN(dy)) return;

  win.setPosition(Math.round(session.startWinX + dx), Math.round(session.startWinY + dy));
});

ipcMain.on('floating-button:drag-end', (event) => {
  dragSessions.delete(event.sender.id);
});

app.whenReady().then(() => {
  createTray();
  createFloatingButtonWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createFloatingButtonWindow();
    }
  });
});

app.on('before-quit', () => {
  if (isQuitting) {
    return;
  }
  isQuitting = true;
  persistFloatingButtonPosition();
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
