import { invoke, listenCompat } from './tauriCore'

export const floatingButtonApi = {
  getConfig: () => invoke('get_floating_button_config'),
  onClick: () => invoke('floating_button_clicked'),
  startDrag: () => invoke('floating_button_drag_start'),
  moveDrag: (dx, dy) => invoke('floating_button_drag_move', { dx, dy }),
  endDrag: () => invoke('floating_button_drag_end'),
  setIgnoreMouseEvents: (ignore) => invoke('floating_button_set_ignore_mouse', { ignore }),
  onConfigUpdated: (callback) => listenCompat('floating-config-updated', callback)
}
