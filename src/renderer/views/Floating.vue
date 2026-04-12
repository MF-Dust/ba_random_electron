<template>
  <FloatingButton
    :size-px="sizePx"
    :transparency-percent="transparencyPercent"
    @click="handleFloatingButtonClick"
  />
</template>

<script setup>
import { ref, onMounted } from 'vue'
import FloatingButton from '../components/FloatingButton.vue'

const sizePx = ref(50)
const transparencyPercent = ref(20)

async function initConfig() {
  if (!window.floatingButtonApi) return
  const cfg = await window.floatingButtonApi.getConfig()
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
})
</script>
