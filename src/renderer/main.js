import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import './style.css'

// If accessed externally in a browser, redirect straight to config!
const isElectron = window.floatingButtonApi !== undefined
  || window.pickCountApi !== undefined
  || window.pickResultApi !== undefined
  || navigator.userAgent.toLowerCase().indexOf('electron') > -1;
if (!isElectron && window.location.hash === '') {
  router.push('/config');
}

createApp(App).use(router).mount('#app')
