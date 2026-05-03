import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import './style.css'

const logToMain = (level, text) => {
  if (window.logApi && typeof window.logApi.send === 'function') {
    window.logApi.send(level, text)
  }
}

['log', 'info', 'warn', 'error'].forEach((method) => {
  const original = console[method].bind(console)
  console[method] = (...args) => {
    const text = args.map(arg => {
      if (typeof arg === 'string') return arg
      try {
        return JSON.stringify(arg)
      } catch (_error) {
        return String(arg)
      }
    }).join(' ')
    logToMain(method === 'log' ? 'info' : method, text)
    original(...args)
  }
})

// If accessed externally in a browser, redirect straight to config!
const isElectron = window.floatingButtonApi !== undefined
  || window.pickCountApi !== undefined
  || window.pickResultApi !== undefined
  || navigator.userAgent.toLowerCase().indexOf('electron') > -1;
if (!isElectron && window.location.hash === '') {
  router.push('/config');
}

createApp(App).use(router).mount('#app')
