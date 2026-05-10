<template>
  <FloatingButton
    :size-px="sizePx"
    :transparency-percent="transparencyPercent"
    @click="handleFloatingButtonClick"
  />
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import FloatingButton from '../components/FloatingButton.vue'

const sizePx = ref(50)
const transparencyPercent = ref(20)

async function initConfig() {
  if (!window.floatingButtonApi) return
  const cfg = await window.floatingButtonApi.getConfig()
  applyConfig(cfg)
}

function applyConfig(cfg) {
  sizePx.value = Math.round(50 * (cfg.sizePercent / 100))
  transparencyPercent.value = cfg.transparencyPercent
}

function handleFloatingButtonClick() {
  if (window.floatingButtonApi) {
    window.floatingButtonApi.onClick()
  }
}

onMounted(() => {
  initConfig()
  if (window.floatingButtonApi && typeof window.floatingButtonApi.onConfigUpdated === 'function') {
    removeConfigListener = window.floatingButtonApi.onConfigUpdated((cfg) => {
      applyConfig(cfg)
    })
  }
})

let removeConfigListener = null

onBeforeUnmount(() => {
  if (typeof removeConfigListener === 'function') {
    removeConfigListener()
  }
})
</script>
