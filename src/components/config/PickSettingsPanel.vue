<template>
  <div class="ba-card-group">
    <div class="ba-card">
      <div class="ba-card-header">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
        </svg>
        <span>音效 & 动画</span>
      </div>

      <div class="ba-toggle-row">
        <n-switch v-model:value="config.pickCountDialog.defaultPlayMusic" />
        <div class="ba-toggle-content">
          <span class="ba-toggle-label">抽选时播放背景音乐</span>
          <span class="ba-toggle-hint">如果老师在上课时用，记得确认一下环境哦～不要社死了！</span>
        </div>
      </div>

      <div class="ba-toggle-row">
        <n-switch v-model:value="config.pickResultDialog.defaultPlayGachaSound" />
        <div class="ba-toggle-content">
          <span class="ba-toggle-label">抽选时播放音效</span>
          <span class="ba-toggle-hint">和音乐一样，上课时请注意音量哦 ♪</span>
        </div>
      </div>

      <div class="ba-form-item">
        <label class="ba-label">音效音量大小</label>
        <div class="ba-slider-row">
          <n-slider
            v-model:value="config.pickResultDialog.gachaSoundVolume"
            :min="0"
            :max="1"
            :step="0.05"
            style="flex: 1;"
          />
          <span class="ba-slider-value">{{ (config.pickResultDialog.gachaSoundVolume * 100).toFixed(0) }}%</span>
        </div>
      </div>
    </div>

    <div class="ba-card">
      <div class="ba-card-header">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
          <line x1="8" y1="21" x2="16" y2="21"/>
          <line x1="12" y1="17" x2="12" y2="21"/>
        </svg>
        <span>画面效果</span>
      </div>

      <div class="ba-form-item">
        <label class="ba-label">背景变暗程度</label>
        <div class="ba-slider-row">
          <n-slider
            v-model:value="config.pickCountDialog.backgroundDarknessPercent"
            :min="0"
            :max="100"
            :step="1"
            style="flex: 1;"
          />
          <span class="ba-slider-value">{{ config.pickCountDialog.backgroundDarknessPercent }}%</span>
        </div>
      </div>

      <div class="ba-form-item">
        <label class="ba-label">每次默认抽几人</label>
        <p class="ba-sublabel">范围 {{ MIN_PICK_COUNT }} - {{ MAX_PICK_COUNT }}，老师按需来就好啦～</p>
        <n-input-number
          v-model:value="config.pickCountDialog.defaultCount"
          :min="MIN_PICK_COUNT"
          :max="MAX_PICK_COUNT"
          style="width: 100%;"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { MAX_PICK_COUNT, MIN_PICK_COUNT } from '../../configDefaults'

defineProps({
  config: {
    type: Object,
    required: true
  }
})
</script>

<style scoped>
.ba-card-group {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.ba-card {
  background: #ffffff;
  border: 1px solid rgba(18, 138, 250, 0.10);
  border-radius: 12px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: box-shadow 0.25s, transform 0.25s;
}

.ba-card:hover {
  box-shadow: 0 4px 20px rgba(18, 138, 250, 0.08);
  transform: translateY(-1px);
}

.ba-card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #128afa;
  font-weight: 700;
  font-size: 15px;
}

.ba-toggle-row {
  display: flex;
  align-items: flex-start;
  gap: 10px;
}

.ba-toggle-content {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.ba-toggle-label {
  font-size: 14px;
  color: #1a3a5c;
  font-weight: 500;
}

.ba-toggle-hint {
  font-size: 12px;
  color: #8ca3bf;
}

.ba-form-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.ba-label {
  font-size: 14px;
  font-weight: 600;
  color: #1a3a5c;
}

.ba-sublabel {
  margin: 0;
  font-size: 12px;
  color: #8ca3bf;
}

.ba-slider-row {
  display: flex;
  align-items: center;
  gap: 14px;
}

.ba-slider-value {
  font-size: 13px;
  font-weight: 700;
  color: #128afa;
  min-width: 40px;
  text-align: right;
}
</style>
