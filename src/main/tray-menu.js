const { Menu } = require('electron');

function buildTrayContextMenu({ onOpenConfig, onRestart, onQuit }) {
  return Menu.buildFromTemplate([
    {
      label: '配置',
      click: () => {
        if (typeof onOpenConfig === 'function') {
          onOpenConfig();
        }
      }
    },
    {
      label: '重启',
      click: () => {
        if (typeof onRestart === 'function') {
          onRestart();
        }
      }
    },
    {
      label: '退出',
      click: () => {
        if (typeof onQuit === 'function') {
          onQuit();
        }
      }
    }
  ]);
}

module.exports = {
  buildTrayContextMenu
};
