<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'

const results = ref([])
const animationKey = ref(0)
const instructionText = ref('点击任意位置关闭')

const topRow = computed(() => results.value.slice(0, 5))
const bottomRow = computed(() => results.value.slice(5))
const isTwoRows = computed(() => results.value.length > 5)

function normalizeResults(payload) {
  const list = Array.isArray(payload?.results) ? payload.results : payload
  if (!Array.isArray(list)) return []
  return list
    .map((item) => {
      if (!item) return null
      if (typeof item === 'string') return { name: item.trim() }
      if (typeof item === 'object') return { name: String(item.name || '').trim() }
      return null
    })
    .filter((item) => item && item.name)
}

function applyResults(payload) {
  results.value = normalizeResults(payload)
  animationKey.value += 1
}

function closeResult() {
  if (window.pickResultApi) {
    window.pickResultApi.close()
  }
}

let removeOpenListener = null

onMounted(async () => {
  if (window.pickResultApi && typeof window.pickResultApi.getResults === 'function') {
    const initial = await window.pickResultApi.getResults()
    applyResults({ results: initial })
  }

  if (window.pickResultApi && typeof window.pickResultApi.onOpen === 'function') {
    removeOpenListener = window.pickResultApi.onOpen((payload) => {
      applyResults(payload)
    })
  }
})

onBeforeUnmount(() => {
  if (typeof removeOpenListener === 'function') {
    removeOpenListener()
  }
})
</script>

<template>
  <div class="result-stage" tabindex="0" @click="closeResult" @keydown.esc="closeResult">
    <div class="result-rows" :class="{ 'is-two-rows': isTwoRows }" :key="animationKey">
      <div class="result-row">
        <div
          v-for="(item, index) in topRow"
          :key="`top-${index}-${item.name}`"
          class="letter-card"
          :style="{ '--index': index }"
        >
          <img class="letter-img" src="/image/letter.png" alt="letter" />
          <div class="name-card">
            <span>{{ item.name }}</span>
          </div>
        </div>
      </div>
      <div v-if="isTwoRows" class="result-row">
        <div
          v-for="(item, index) in bottomRow"
          :key="`bottom-${index}-${item.name}`"
          class="letter-card"
          :style="{ '--index': index + 5 }"
        >
          <img class="letter-img" src="/image/letter.png" alt="letter" />
          <div class="name-card">
            <span>{{ item.name }}</span>
          </div>
        </div>
      </div>
    </div>
    <p v-if="results.length" class="result-hint">{{ instructionText }}</p>
    <p v-else class="result-empty">暂无抽取结果</p>
  </div>
</template>

<style scoped>
.result-stage {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 26px;
  background: rgba(0, 0, 0, 0.35);
  outline: none;
}

.result-rows {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 36px;
}

.result-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 28px;
}

.letter-card {
  position: relative;
  width: clamp(120px, 16vw, 200px);
  aspect-ratio: 4 / 3;
  opacity: 0;
  transform: scale(2.5) rotate(15deg);
  animation: letter-fly-in 0.6s ease-out forwards;
  animation-delay: calc(var(--index) * 0.12s);
}

.letter-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  filter: drop-shadow(0 12px 24px rgba(0, 0, 0, 0.25));
}

.name-card {
  position: absolute;
  inset: 18% 10%;
  background: #ffffff;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 6px 10px;
  font-size: clamp(16px, 2.1vw, 26px);
  font-weight: 700;
  color: #1c2741;
  box-shadow: 0 10px 26px rgba(0, 0, 0, 0.25);
  opacity: 0;
  transform: translateY(12px) scale(0.96);
  animation: name-reveal 0.45s ease-out forwards;
  animation-delay: calc(var(--index) * 0.12s + 0.7s);
}

.result-hint {
  margin: 0;
  font-size: 16px;
  color: rgba(255, 255, 255, 0.8);
  letter-spacing: 2px;
}

.result-empty {
  margin: 0;
  font-size: 20px;
  color: rgba(255, 255, 255, 0.75);
  letter-spacing: 2px;
}

@keyframes letter-fly-in {
  0% {
    opacity: 0;
    transform: scale(2.5) rotate(15deg) translateY(-24px);
  }
  100% {
    opacity: 1;
    transform: scale(1) rotate(15deg) translateY(0);
  }
}

@keyframes name-reveal {
  0% {
    opacity: 0;
    transform: translateY(12px) scale(0.96);
  }
  100% {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
</style>
