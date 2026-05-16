<script setup>
import { usePickResultDialog } from '../composables/usePickResultDialog'

const {
  results,
  animationKey,
  instructionText,
  revealStarted,
  isClosing,
  topRow,
  bottomRow,
  isTwoRows,
  handleStageClick,
  handleKeydown
} = usePickResultDialog()
</script>

<template>
  <div
    class="result-stage"
    :class="{ 'is-closing': isClosing }"
    tabindex="0"
    @click="handleStageClick"
    @contextmenu.prevent
    @keydown="handleKeydown"
  >
    <div v-if="results.length" class="quick-result">

      <div class="result-rows" :class="{ 'is-two-rows': isTwoRows }" :key="animationKey">
        <div class="result-row">
          <div
            v-for="(item, index) in topRow"
            :key="`top-${index}-${item.name}`"
            class="letter-card"
            :class="`is-${item.rarity}`"
            :style="{ '--index': index }"
          >
            <img class="letter-img" :src="`/image/${item.rarity}.png`" alt="letter" />
            <div class="name-card" :class="{ 'is-reveal': revealStarted }" :style="{ '--reveal-index': index }">
              <span>{{ item.name }}</span>
            </div>
          </div>
        </div>
        <div v-if="isTwoRows" class="result-row">
          <div
            v-for="(item, index) in bottomRow"
            :key="`bottom-${index}-${item.name}`"
            class="letter-card"
            :class="`is-${item.rarity}`"
            :style="{ '--index': index + 5 }"
          >
            <img class="letter-img" :src="`/image/${item.rarity}.png`" alt="letter" />
            <div class="name-card" :class="{ 'is-reveal': revealStarted }" :style="{ '--reveal-index': index + 5 }">
              <span>{{ item.name }}</span>
            </div>
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

.result-stage.is-closing {
  pointer-events: none;
  animation: result-fade-out 220ms ease forwards;
}

.quick-result {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 18px;
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
  animation: none;
  border: 3px solid transparent;
}

.letter-card.is-blue .name-card {
  border-color: #76c7ff;
  box-shadow: 0 0 12px 2px rgba(118, 199, 255, 0.5), 0 10px 26px rgba(0, 0, 0, 0.25);
}

.letter-card.is-gold .name-card {
  border-color: #ffd84d;
  box-shadow: 0 0 12px 2px rgba(255, 216, 77, 0.5), 0 10px 26px rgba(0, 0, 0, 0.25);
}

.letter-card.is-pink .name-card {
  border-color: #ff7ee2;
  box-shadow: 0 0 12px 2px rgba(255, 126, 226, 0.5), 0 10px 26px rgba(0, 0, 0, 0.25);
}

.name-card.is-reveal {
  animation: name-reveal 0.45s ease-out forwards;
  animation-delay: calc(var(--reveal-index) * 0.12s + 0.1s);
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

@keyframes result-fade-out {
  0% {
    opacity: 1;
  }
  100% {
    opacity: 0;
  }
}
</style>
