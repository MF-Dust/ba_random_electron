function byId(id) {
  return document.getElementById(id);
}

function maybeNumber(value) {
  if (value === '' || value === null || value === undefined) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
}

async function fetchConfig() {
  const response = await fetch('/api/config');
  if (!response.ok) throw new Error('加载配置失败');
  return response.json();
}

function fillForm(config) {
  byId('fb-size').value = config.floatingButton.sizePercent;
  byId('fb-transparency').value = config.floatingButton.transparencyPercent;
  byId('fb-top').checked = Boolean(config.floatingButton.alwaysOnTop);
  byId('fb-pos-x').value = config.floatingButton.position.x ?? '';
  byId('fb-pos-y').value = config.floatingButton.position.y ?? '';

  byId('pick-music').checked = Boolean(config.pickCountDialog.defaultPlayMusic);
  byId('pick-dark').value = config.pickCountDialog.backgroundDarknessPercent;
  byId('pick-count').value = config.pickCountDialog.defaultCount;

  byId('web-port').value = config.webConfig.port;
}

function collectConfig() {
  return {
    floatingButton: {
      sizePercent: Number(byId('fb-size').value),
      transparencyPercent: Number(byId('fb-transparency').value),
      alwaysOnTop: byId('fb-top').checked,
      position: {
        x: maybeNumber(byId('fb-pos-x').value),
        y: maybeNumber(byId('fb-pos-y').value)
      }
    },
    pickCountDialog: {
      defaultPlayMusic: byId('pick-music').checked,
      backgroundDarknessPercent: Number(byId('pick-dark').value),
      defaultCount: Number(byId('pick-count').value)
    },
    webConfig: {
      port: Number(byId('web-port').value)
    }
  };
}

async function saveConfig(config) {
  const response = await fetch('/api/config', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(config)
  });
  if (!response.ok) throw new Error('保存失败');
  return response.json();
}

async function restartApp() {
  await fetch('/api/restart', { method: 'POST' });
}

async function init() {
  const form = byId('config-form');
  const config = await fetchConfig();
  fillForm(config);

  form.addEventListener('submit', async (event) => {
    event.preventDefault();

    try {
      const payload = collectConfig();
      await saveConfig(payload);

      const shouldRestart = window.confirm('配置已保存。是否立即重启应用以应用配置？');
      if (shouldRestart) {
        await restartApp();
      }
    } catch (error) {
      window.alert('保存失败，请检查输入内容。');
    }
  });
}

init().catch(() => {
  window.alert('配置页面初始化失败。');
});
