import { createRouter, createWebHashHistory } from 'vue-router'

import Floating from '../views/Floating.vue'
import PickCount from '../views/PickCount.vue'
import PickResult from '../views/PickResult.vue'
import WebConfig from '../views/WebConfig.vue'

const routes = [
  { path: '/', component: Floating },
  { path: '/pick-count', component: PickCount },
  { path: '/pick-result', component: PickResult },
  { path: '/config', component: WebConfig }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
