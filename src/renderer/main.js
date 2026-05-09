import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { installTauriCompatApis } from './tauriApi'

import './style.css'

installTauriCompatApis()

const logToMain = (level, text) => {
  if (window.logApi && typeof window.logApi.send === 'function') {
    window.logApi.send(level, text)
  }
}

const recentConsoleLogs = new Map()
const shouldForwardConsoleLog = (level, text) => {
  const now = Date.now()
  const key = `${level}:${text}`
  const last = recentConsoleLogs.get(key) || 0
  recentConsoleLogs.set(key, now)
  if (recentConsoleLogs.size > 80) {
    const cutoff = now - 10_000
    for (const [entryKey, entryTime] of recentConsoleLogs) {
      if (entryTime < cutoff) {
        recentConsoleLogs.delete(entryKey)
      }
    }
  }
  return now - last > 1000
}

['warn', 'error'].forEach((method) => {
  const original = console[method].bind(console)
  console[method] = (...args) => {
    const text = args.map(arg => {
      if (typeof arg === 'string') return arg
      try {
        return JSON.stringify(arg)
      } catch (_error) {
        return String(arg)
      }
    }).join(' ').slice(0, 800)
    if (shouldForwardConsoleLog(method, text)) {
      logToMain(method, text)
    }
    original(...args)
  }
})

// If accessed outside the Tauri shell during frontend preview, redirect straight to config.
const isElectron = window.floatingButtonApi !== undefined
  || window.pickCountApi !== undefined
  || window.pickResultApi !== undefined
  || window.__TAURI_INTERNALS__ !== undefined;
if (!isElectron && window.location.hash === '') {
  router.push('/config');
}

createApp(App).use(router).mount('#app')
