import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const listenCompat = (eventName, callback) => {
  let unlisten = null
  let disposed = false

  listen(eventName, (event) => {
    callback(event.payload)
  }).then((fn) => {
    if (disposed) {
      fn()
      return
    }
    unlisten = fn
  }).catch((error) => {
    console.warn(`Failed to listen ${eventName}`, error)
  })

  return () => {
    disposed = true
    if (typeof unlisten === 'function') {
      unlisten()
    }
  }
}

export const appApi = {
  getConfig: () => invoke('get_config'),
  parseStudentListText: (rawText, existingStudents) => invoke('parse_student_list_text', { rawText, existingStudents }),
  importStudentListFromFile: (existingStudents) => invoke('import_student_list_from_file', { existingStudents }),
  saveConfig: (config) => invoke('save_app_config', { config }),
  getAppInfo: () => invoke('get_app_info'),
  checkUpdate: () => invoke('check_update'),
  requestAdminElevation: () => invoke('request_admin_elevation'),
  createAdminStartupTask: (exePath, taskName) => invoke('create_admin_startup_task', { exePath, taskName }),
  getLogs: () => invoke('get_logs'),
  onLogEntry: (callback) => listenCompat('log-entry', callback)
}

export function installTauriCompatApis() {
  window.floatingButtonApi = {
    getConfig: () => invoke('get_floating_button_config'),
    onClick: () => invoke('floating_button_clicked'),
    startDrag: () => invoke('floating_button_drag_start'),
    moveDrag: (dx, dy) => invoke('floating_button_drag_move', { dx, dy }),
    endDrag: () => invoke('floating_button_drag_end'),
    setIgnoreMouseEvents: (ignore) => invoke('floating_button_set_ignore_mouse', { ignore }),
    onConfigUpdated: (callback) => listenCompat('floating-config-updated', callback)
  }

  window.pickCountApi = {
    getConfig: () => invoke('get_pick_count_config'),
    cancel: () => invoke('cancel_pick_count'),
    confirm: (count, playMusic) => invoke('confirm_pick_count', { count, playMusic }),
    onOpen: (callback) => listenCompat('pick-count-open', callback),
    onStopBgm: (callback) => listenCompat('pick-count-stop-bgm', callback)
  }

  window.pickResultApi = {
    getResults: () => invoke('get_pick_results'),
    getConfig: () => invoke('get_pick_result_config'),
    close: () => invoke('close_pick_result'),
    onOpen: (callback) => listenCompat('pick-result-open', callback),
    onReset: (callback) => listenCompat('pick-result-reset', callback)
  }

  window.audioApi = {
    playClickSound: () => invoke('play_click_sound'),
    playBgm: () => invoke('play_bgm'),
    stopBgm: () => invoke('stop_bgm'),
    playGachaSound: (volume) => invoke('play_gacha_sound', { volume }),
    stopGachaSound: () => invoke('stop_gacha_sound')
  }

  window.logApi = {
    send: (level, text) => {
      const safeText = String(text || '').slice(0, 800)
      if (!safeText) return
      invoke('renderer_log', { level, text: safeText }).catch(() => {})
    }
  }
}
