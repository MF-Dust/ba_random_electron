<script setup>
import { usePickCountDialog } from '../composables/usePickCountDialog'

const {
  MIN_COUNT,
  MAX_COUNT,
  count,
  playMusic,
  isLeaving,
  canDecrease,
  canIncrease,
  overlayStyle,
  increaseCount,
  decreaseCount,
  setMinCount,
  setMaxCount,
  handleCancel,
  handleConfirm
} = usePickCountDialog()
</script>

<template>
  <div class="pick-overlay" :class="{ 'is-leaving': isLeaving }" :style="overlayStyle">
    <div class="pick-panel">
      <h1 class="pick-title">要点名几个人呢～</h1>

      <div class="pick-counter-row">
        <button class="pick-circle-btn" :disabled="isLeaving || !canDecrease" @click="decreaseCount" aria-label="减少～">-</button>
        <div class="pick-count-box">{{ count }}</div>
        <button class="pick-circle-btn" :disabled="isLeaving || !canIncrease" @click="increaseCount" aria-label="增加！">+</button>
      </div>

      <div class="pick-range-row">
        <button class="pick-range-btn" :disabled="isLeaving || !canDecrease" @click="setMinCount">最少</button>
        <span class="pick-range-hint">可选范围 {{ MIN_COUNT }} - {{ MAX_COUNT }}，老师看着办～</span>
        <button class="pick-range-btn" :disabled="isLeaving || !canIncrease" @click="setMaxCount">最多</button>
      </div>

      <div class="pick-actions">
        <button class="pick-action-btn pick-action-cancel" :disabled="isLeaving" @click="handleCancel">先不要了</button>
        <button class="pick-action-btn pick-action-confirm" :disabled="isLeaving" @click="handleConfirm">开始点名！</button>
      </div>

      <label class="pick-music-row">
        <input class="pick-checkbox" type="checkbox" v-model="playMusic" :disabled="isLeaving" />
        <span>播放超～喜庆的点名BGM！</span>
      </label>
    </div>
  </div>
</template>

<style scoped>
.pick-overlay {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  animation: pick-overlay-fade-in 0.4s ease forwards;
}

.pick-overlay.is-leaving {
  animation: pick-overlay-fade-out 0.4s ease forwards;
}

.pick-panel {
  width: min(560px, 92vw);
  border-radius: 20px;
  padding: 30px 30px 20px;
  background: #eff6ff;
  border: 1px solid rgba(255, 255, 255, 0.76);
  box-shadow:
    0 24px 68px rgba(3, 17, 44, 0.42),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
  opacity: 0;
  transform: translateY(18px);
  animation: pick-panel-fly-fade-in 0.4s ease-out forwards;
}

.pick-overlay.is-leaving .pick-panel {
  animation: pick-panel-fly-fade-out 0.4s ease-in forwards;
}

.pick-title {
  margin: 0;
  text-align: center;
  color: #0b325f;
  font-size: 32px;
  line-height: 1;
  letter-spacing: 3px;
  font-weight: 800;
  text-shadow: 0 2px 10px rgba(255, 255, 255, 0.45);
}

.pick-counter-row {
  margin-top: 26px;
  display: grid;
  grid-template-columns: 80px 1fr 80px;
  align-items: center;
  gap: 14px;
}

.pick-range-row {
  margin-top: 12px;
  display: grid;
  grid-template-columns: 88px 1fr 88px;
  align-items: center;
  gap: 12px;
}

.pick-range-btn {
  height: 36px;
  border-radius: 12px;
  border: 0;
  cursor: pointer;
  font-size: 14px;
  font-weight: 700;
  color: #0b2a55;
  background: #e3e9f3;
  box-shadow: 0 10px 16px rgba(19, 35, 62, 0.2);
  transition: transform 120ms ease, filter 120ms ease, box-shadow 120ms ease;
}

.pick-range-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  filter: brightness(1.05);
}

.pick-range-btn:active:not(:disabled) {
  transform: translateY(1px) scale(0.985);
}

.pick-range-btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
  box-shadow: none;
}

.pick-range-hint {
  text-align: center;
  color: rgba(15, 35, 66, 0.78);
  font-size: 13px;
  letter-spacing: 0.3px;
}

.pick-circle-btn {
  width: 70px;
  height: 70px;
  border-radius: 15%;
  border: none;
  cursor: pointer;
  font-size: 60px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #101a8c;
  background: #cfd7e4;
  box-shadow: 0 14px 28px rgba(10, 30, 68, 0.38);
  transition: transform 120ms ease, filter 120ms ease, box-shadow 120ms ease;
}

.pick-circle-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  filter: brightness(1.08);
  box-shadow: 0 18px 30px rgba(9, 27, 58, 0.42);
}

.pick-circle-btn:active:not(:disabled) {
  transform: translateY(1px) scale(0.985);
}

.pick-circle-btn:disabled {
  cursor: not-allowed;
  background: linear-gradient(180deg, #aab3be, #969faa);
  color: rgba(255, 255, 255, 0.7);
  box-shadow: none;
}

.pick-count-box {
  height: 76px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #0d1356;
  border: 1px solid rgba(161, 204, 255, 0.42);
  color: #eff6ff;
  font-size: 44px;
  font-weight: 700;
  letter-spacing: 2px;
  box-shadow: inset 0 1px 10px rgba(115, 180, 255, 0.32);
}

.pick-actions {
  margin-top: 24px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.pick-action-btn {
  height: 50px;
  border-radius: 13px;
  border: 0;
  cursor: pointer;
  font-size: 19px;
  font-weight: 700;
  letter-spacing: 1px;
  transition: transform 120ms ease, filter 120ms ease, box-shadow 120ms ease;
}

.pick-action-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  filter: brightness(1.04);
}

.pick-action-btn:active:not(:disabled) {
  transform: translateY(1px) scale(0.992);
}

.pick-action-btn:disabled {
  cursor: not-allowed;
  opacity: 0.7;
  filter: grayscale(0.15);
  box-shadow: none;
}

.pick-action-cancel {
  color: #152131;
  background: #fdfeff;
  box-shadow: 0 10px 18px rgba(22, 31, 50, 0.2);
}

.pick-action-confirm {
  color: #3a2800;
  background: #fffb13e2;
  box-shadow: 0 10px 18px rgba(92, 68, 0, 0.28);
}

.pick-music-row {
  margin-top: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 9px;
  color: rgba(17, 39, 71, 0.9);
  font-size: 14px;
  letter-spacing: 0.4px;
}

.pick-checkbox {
  width: 17px;
  height: 17px;
  accent-color: #2662ad;
}

@keyframes pick-overlay-fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes pick-overlay-fade-out {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

@keyframes pick-panel-fly-fade-in {
  from {
    opacity: 0;
    transform: translateY(18px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes pick-panel-fly-fade-out {
  from {
    opacity: 1;
    transform: translateY(0);
  }
  to {
    opacity: 0;
    transform: translateY(22px);
  }
}
</style>
