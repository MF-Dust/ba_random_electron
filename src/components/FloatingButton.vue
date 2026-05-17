<template>
  <div class="floating-root">
    <button
      class="floating-button"
      :class="{ 'is-dragging': isDragging }"
      :style="buttonStyle"
      @contextmenu.prevent
      @pointerdown="handlePointerDown"
      @pointermove="handlePointerMove"
      @pointerup="handlePointerUp"
      @pointercancel="handlePointerCancel"
      title="点名点名～"
    >
      <img src="/image/random.svg" alt="阿罗娜的点名按钮" draggable="false" />
    </button>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useFloatingDrag } from '../composables/useFloatingDrag'

const props = defineProps({
  sizePx: {
    type: Number,
    required: true
  },
  transparencyPercent: {
    type: Number,
    required: true
  }
})

const emit = defineEmits(['click'])

const styleOpacity = computed(() => {
  return Math.max(0, Math.min(1, 1 - props.transparencyPercent / 100))
})

const buttonStyle = computed(() => {
  return {
    width: `${props.sizePx}px`,
    height: `${props.sizePx}px`,
    opacity: String(styleOpacity.value)
  }
})


const {
  isDragging,
  handlePointerDown,
  handlePointerMove,
  handlePointerUp,
  handlePointerCancel
} = useFloatingDrag(emit)
</script>

<style scoped>
.floating-root {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.floating-button {
  position: relative;
  border: 0;
  border-radius: 16px;
  cursor: pointer;
  touch-action: none;
  padding: 10px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(145deg, #66CCFFF0, rgba(64, 145, 240, 0.95));
  transition: transform 300ms ease, box-shadow 300ms ease;
}

.floating-button:hover {
  transform: translateY(-1px);
  box-shadow: 1px 10px 5px rgba(0, 0, 0, 0.1);
}

.floating-button:active {
  transform: translateY(1px) scale(0.985);
}

.floating-button.is-dragging,
.floating-button.is-dragging:hover,
.floating-button.is-dragging:active {
  transform: none;
  transition: none;
}

.floating-button img {
  width: 120%;
  height: 120%;
  object-fit: contain;
  pointer-events: none;
}

</style>
