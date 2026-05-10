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
import { floatingButtonApi } from '../api/floatingButtonApi'

const sizePx = ref(50)
const transparencyPercent = ref(20)

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
  removeConfigListener = floatingButtonApi.onConfigUpdated((cfg) => {
    applyConfig(cfg)
  })
})

let removeConfigListener = null

onBeforeUnmount(() => {
  if (typeof removeConfigListener === 'function') {
    removeConfigListener()
  }
})
</script>
