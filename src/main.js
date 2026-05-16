import { createApp } from 'vue'
import naive from 'naive-ui'
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

;['warn', 'error'].forEach((method) => {
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
    logToMain(method, text)
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

createApp(App).use(router).use(naive).mount('#app')
