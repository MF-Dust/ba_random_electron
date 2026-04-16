const { app, BrowserWindow, Tray, nativeImage, shell, dialog, ipcMain } = require('electron');
const http = require('http');
const fs = require('fs');
const path = require('path');
const yaml = require('js-yaml');
const { buildTrayContextMenu } = require('./tray-menu');

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
    defaultCount: 1
  },
  webConfig: {
    port: 21219
  }
};

let currentConfig = DEFAULT_CONFIG;
const dragSessions = new Map();
let appTray = null;
let floatingButtonWindow = null;
let pickCountWindow = null;
let isPickCountWindowReady = false;
let isFloatingHiddenForPickCount = false;
let configServer = null;
let configServerPort = null;
let isQuitting = false;
let floatingWindowWatchdog = null;
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
  const web = source.webConfig && typeof source.webConfig === 'object' ? source.webConfig : {};

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
      defaultCount: Math.round(
        clampNumber(
          pick.defaultCount,
          1,
          10,
          DEFAULT_CONFIG.pickCountDialog.defaultCount
        )
      )
    },
    webConfig: {
      port: Math.round(clampNumber(web.port, 1, 65535, DEFAULT_CONFIG.webConfig.port))
    }
  };
}

function getConfigPath() {
  return path.join(process.cwd(), 'config.yml');
}

function toConfigYamlWithComments(config) {
  const fb = config.floatingButton;
  const pick = config.pickCountDialog;
  const web = config.webConfig;
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
    '  # 人数默认值，范围 1-10 的整数，默认 1',
    `  defaultCount: ${pick.defaultCount}`,
    '',
    '# 网页配置服务',
    'webConfig:',
    '  # 配置网页端口（默认 21219）',
    `  port: ${web.port}`,
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

function refreshConfig() {
  currentConfig = loadConfig();
  return currentConfig;
}

function getMimeType(filePath) {
  if (filePath.endsWith('.html')) return 'text/html; charset=utf-8';
  if (filePath.endsWith('.js')) return 'application/javascript; charset=utf-8';
  if (filePath.endsWith('.css')) return 'text/css; charset=utf-8';
  if (filePath.endsWith('.json')) return 'application/json; charset=utf-8';
  return 'text/plain; charset=utf-8';
}

function sendJson(res, statusCode, payload) {
  res.writeHead(statusCode, {
    'Content-Type': 'application/json; charset=utf-8'
  });
  res.end(JSON.stringify(payload));
}

function parseRequestJsonBody(req) {
  return new Promise((resolve, reject) => {
    let body = '';
    req.on('data', (chunk) => {
      body += chunk;
      if (body.length > 1024 * 1024) {
        reject(new Error('Payload too large'));
      }
    });

    req.on('end', () => {
      if (!body.trim()) {
        resolve({});
        return;
      }
      try {
        resolve(JSON.parse(body));
      } catch (error) {
        reject(error);
      }
    });

    req.on('error', reject);
  });
}

function openConfigPageInBrowser() {
  const config = refreshConfig();
  const url = `http://localhost:${config.webConfig.port}`;
  shell.openExternal(url);
}

function createConfigServerRequestHandler() {
  return async (req, res) => {
    const requestUrl = req.url || '/';
    const rendererDir = path.join(__dirname, '../renderer', 'web-config');

    if (req.method === 'GET' && requestUrl === '/api/config') {
      return sendJson(res, 200, refreshConfig());
    }

    if (req.method === 'POST' && requestUrl === '/api/config') {
      try {
        const payload = await parseRequestJsonBody(req);
        const normalized = normalizeConfig(payload);
        currentConfig = normalized;
        saveConfig(normalized);
        
        // 动态重启服务和窗口来免重启应用
        startConfigServer();
        if (floatingButtonWindow && !floatingButtonWindow.isDestroyed()) {
          // 直接关闭窗口将触发 close 事件，它会自动延时 60ms 重新创建一个完美应用了新尺寸及样式的新窗口。
          // 这完全规避了 Windows 下无边框透明窗口直接 setSize 会渲染崩溃甚至消失的底层 Bug，
          // 也避免了开发模式下 app.relaunch() 抛弃 Vite 代理的问题。
          floatingButtonWindow.close();
        }
        
        return sendJson(res, 200, {
          ok: true,
          message: '配置保存成功，悬浮窗已自动刷新配置',
          restartRequired: false
        });
      } catch (error) {
        return sendJson(res, 400, {
          ok: false,
          message: '配置保存失败，请检查输入格式'
        });
      }
    }

    if (req.method === 'POST' && requestUrl === '/api/restart') {
      sendJson(res, 200, { ok: true });
      setTimeout(() => {
        isQuitting = true;
        app.relaunch();
        app.exit(0);
      }, 80);
      return;
    }

    const urlPath = requestUrl.split('?')[0].split('#')[0];

    if (!urlPath.startsWith('/api')) {
        if (process.env.VITE_DEV_SERVER_URL) {
            res.writeHead(302, { Location: process.env.VITE_DEV_SERVER_URL + '#/config' });
            res.end();
            return;
        }

        const distDir = path.join(__dirname, '../dist');
        const targetPath = path.join(distDir, urlPath === '/' ? 'index.html' : urlPath);
        
        if (!targetPath.startsWith(distDir)) {
          return sendJson(res, 403, { ok: false, message: 'Forbidden' });
        }
        
        if (fs.existsSync(targetPath) && fs.statSync(targetPath).isFile()) {
          const fileContent = fs.readFileSync(targetPath);
          res.writeHead(200, { 'Content-Type': getMimeType(targetPath) });
          res.end(fileContent);
          return;
        }
    }

    sendJson(res, 404, { ok: false, message: 'Not Found' });
  };
}

function startConfigServer() {
  const config = refreshConfig();
  const desiredPort = config.webConfig.port;

  if (configServer && configServerPort === desiredPort) {
    return;
  }

  if (configServer) {
    configServer.close();
    configServer = null;
    configServerPort = null;
  }

  const server = http.createServer(createConfigServerRequestHandler());
  server.listen(desiredPort, '127.0.0.1', () => {
    configServerPort = desiredPort;
    console.log(`Config web server running at http://localhost:${desiredPort}`);
  });

  server.on('error', (error) => {
    console.error('Failed to start config web server:', error);
  });

  configServer = server;
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
  if (floatingButtonWindow && !floatingButtonWindow.isDestroyed()) {
    return floatingButtonWindow;
  }

  currentConfig = refreshConfig();
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
    type: 'toolbar', // 防止被托盘或系统当作普通窗口隐藏
    focusable: process.platform !== 'win32', // Windows下设为false以防焦点抢夺导致的隐藏Bug，但仍能接收点击
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

  // 强制最高层级置顶，解决因为设置了 type: 'toolbar' 或 focusable 导致置顶失效的问题
  if (config.floatingButton.alwaysOnTop) {
    win.setAlwaysOnTop(true, 'screen-saver');
  }

  win.setMenuBarVisibility(false);
  if (process.env.VITE_DEV_SERVER_URL) {
    win.loadURL(process.env.VITE_DEV_SERVER_URL);
  } else {
    // When built, Vite outputs renderer to dist (not renderer)
    win.loadFile(path.join(__dirname, '../dist/index.html'));
  }
  win.webContents.on('context-menu', (event) => {
    event.preventDefault();
  });

  win.on('hide', () => {
    if (isQuitting || isFloatingHiddenForPickCount) {
      return;
    }

    setTimeout(() => {
      if (!floatingButtonWindow || floatingButtonWindow.isDestroyed()) {
        return;
      }
      if (isQuitting || isFloatingHiddenForPickCount) {
        return;
      }

      if (!floatingButtonWindow.isVisible()) {
        floatingButtonWindow.setOpacity(1);
        floatingButtonWindow.show();
      }
    }, 0);
  });

  win.on('closed', () => {
    floatingButtonWindow = null;

    if (!isQuitting && !isFloatingHiddenForPickCount) {
      setTimeout(() => {
        if (!isQuitting && !isFloatingHiddenForPickCount) {
          createFloatingButtonWindow();
        }
      }, 60);
    }
  });

  return win;
}

function startFloatingWindowWatchdog() {
  if (floatingWindowWatchdog) {
    clearInterval(floatingWindowWatchdog);
    floatingWindowWatchdog = null;
  }

  floatingWindowWatchdog = setInterval(() => {
    if (isQuitting || isFloatingHiddenForPickCount) {
      return;
    }

    if (!floatingButtonWindow || floatingButtonWindow.isDestroyed()) {
      createFloatingButtonWindow();
      return;
    }

    if (!floatingButtonWindow.isVisible()) {
      floatingButtonWindow.setOpacity(1);
      floatingButtonWindow.show();
    }
  }, 450);
}

function closePickCountWindow() {
  if (!pickCountWindow || pickCountWindow.isDestroyed()) {
    fadeInFloatingButtonWindow();
    return;
  }

  if (pickCountWindow.isVisible()) {
    pickCountWindow.hide();
  }
  isFloatingHiddenForPickCount = false;
  fadeInFloatingButtonWindow();
}

function createPickCountWindowInstance() {
  if (pickCountWindow && !pickCountWindow.isDestroyed()) {
    return;
  }

  const win = new BrowserWindow({
    show: false,
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
  isPickCountWindowReady = false;
  win.setMenuBarVisibility(false);
  if (process.env.VITE_DEV_SERVER_URL) {
    win.loadURL(`${process.env.VITE_DEV_SERVER_URL}#/pick-count`);
  } else {
    // Cannot load file with hash cleanly via loadFile, use loadURL with file protocol
    win.loadURL(`file://${path.join(__dirname, '../dist/index.html')}#/pick-count`);
  }

  win.once('ready-to-show', () => {
    isPickCountWindowReady = true;
  });

  win.on('closed', () => {
    pickCountWindow = null;
    isPickCountWindowReady = false;
    if (!isQuitting) {
      fadeInFloatingButtonWindow();
    }
  });
}

function createPickCountWindow() {
  createPickCountWindowInstance();

  if (!pickCountWindow || pickCountWindow.isDestroyed()) {
    return;
  }

  const openPickCountWindow = () => {
    if (!pickCountWindow || pickCountWindow.isDestroyed()) {
      return;
    }
    pickCountWindow.webContents.send('pick-count:open');
    pickCountWindow.show();
    pickCountWindow.focus();
  };

  if (isPickCountWindowReady) {
    openPickCountWindow();
  } else {
    pickCountWindow.once('ready-to-show', openPickCountWindow);
  }

  isFloatingHiddenForPickCount = true;
  fadeOutFloatingButtonWindow();
}

function createTray() {
  const isDev = !!process.env.VITE_DEV_SERVER_URL;
  // During Vite dev, __dirname is 'dist-electron'
  const trayIconPath = isDev
    ? path.join(__dirname, '../public/image/tray.png')
    : path.join(__dirname, '../dist/image/tray.png');
  const trayIcon = nativeImage.createFromPath(trayIconPath);
  appTray = new Tray(trayIcon);

  appTray.setToolTip('BA Random Electron');
  const trayMenu = buildTrayContextMenu({
    onOpenConfig: () => {
      openConfigPageInBrowser();
    },
    onQuit: () => {
      app.quit();
    }
  });
  appTray.setContextMenu(trayMenu);
}

ipcMain.handle('floating-button:get-config', () => {
  return refreshConfig().floatingButton;
});

ipcMain.on('floating-button:clicked', () => {
  createPickCountWindow();
});

ipcMain.handle('pick-count:get-config', () => {
  return refreshConfig().pickCountDialog;
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
    startWinY: bounds.y,
    width: bounds.width,
    height: bounds.height
  });
});

ipcMain.on('floating-button:drag-move', (event, payload) => {
  const win = BrowserWindow.fromWebContents(event.sender);
  const session = dragSessions.get(event.sender.id);
  if (!win || !session || !payload) return;

  const dx = Number(payload.dx);
  const dy = Number(payload.dy);
  if (Number.isNaN(dx) || Number.isNaN(dy)) return;

  win.setBounds({
    x: Math.round(session.startWinX + dx),
    y: Math.round(session.startWinY + dy),
    width: session.width,
    height: session.height
  });
});

ipcMain.on('floating-button:drag-end', (event) => {
  dragSessions.delete(event.sender.id);
});

app.whenReady().then(() => {
  startConfigServer();
  createTray();
  createFloatingButtonWindow();
  createPickCountWindowInstance();
  startFloatingWindowWatchdog();

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
  if (floatingWindowWatchdog) {
    clearInterval(floatingWindowWatchdog);
    floatingWindowWatchdog = null;
  }
  persistFloatingButtonPosition();
});

app.on('window-all-closed', () => {
  // Keep app resident in tray; explicit quit should come from tray menu.
});
