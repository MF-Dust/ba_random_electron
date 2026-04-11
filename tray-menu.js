const { Menu } = require('electron');

function buildTrayContextMenu({ onOpenConfig, onQuit }) {
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
