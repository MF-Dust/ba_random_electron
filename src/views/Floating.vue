<template>
  <FloatingButton
    :size-px="sizePx"
    :transparency-percent="transparencyPercent"
    @click="handleFloatingButtonClick"
  />
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { floatingButtonApi } from '../api/floatingButtonApi'
import FloatingButton from '../components/FloatingButton.vue'

const sizePx = ref(50)
const transparencyPercent = ref(20)
let removeConfigListener = null
let prewarmTimer = null

async function initConfig() {
  const cfg = await floatingButtonApi.getConfig()
  applyConfig(cfg)
}

function applyConfig(cfg) {
  sizePx.value = Math.round(50 * (cfg.sizePercent / 100))
  transparencyPercent.value = cfg.transparencyPercent
}

function handleFloatingButtonClick() {
  floatingButtonApi.onClick()
}

onMounted(() => {
  initConfig()
  prewarmTimer = window.setTimeout(() => {
    floatingButtonApi.prewarmAuxWindows().catch(() => {})
    prewarmTimer = null
  }, 800)
  removeConfigListener = floatingButtonApi.onConfigUpdated((cfg) => {
    applyConfig(cfg)
  })
})

onBeforeUnmount(() => {
  if (prewarmTimer) {
    window.clearTimeout(prewarmTimer)
    prewarmTimer = null
  }
  if (typeof removeConfigListener === 'function') {
    removeConfigListener()
  }
})
</script>
