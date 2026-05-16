import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  { path: '/', component: () => import('../views/Floating.vue') },
  { path: '/pick-count', component: () => import('../views/PickCount.vue') },
  { path: '/pick-result', component: () => import('../views/PickResult.vue') },
  { path: '/config', component: () => import('../views/WebConfig.vue') }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
