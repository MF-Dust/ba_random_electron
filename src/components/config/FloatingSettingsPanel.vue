<template>
  <div class="ba-card-group">
    <div class="ba-card">
      <div class="ba-card-header">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
          <circle cx="12" cy="12" r="3"/>
        </svg>
        <span>悬浮按钮设置</span>
      </div>

      <div class="ba-form-item">
        <label class="ba-label">悬浮按钮大小</label>
        <p class="ba-sublabel">以 50×50 px 为 100，范围 0-1000</p>
        <n-input-number
          v-model:value="config.floatingButton.sizePercent"
          :min="0"
          :max="1000"
          placeholder="100"
          style="width: 100%;"
        />
      </div>

      <div class="ba-form-item">
        <label class="ba-label">悬浮按钮透明度</label>
        <p class="ba-sublabel">范围 0-100</p>
        <n-slider
          v-model:value="config.floatingButton.transparencyPercent"
          :min="0"
          :max="100"
          :step="1"
        />
        <div class="ba-slider-value">{{ config.floatingButton.transparencyPercent }}%</div>
      </div>

      <div class="ba-toggle-row">
        <n-switch v-model:value="config.floatingButton.alwaysOnTop" />
        <span class="ba-toggle-label">始终置顶显示</span>
      </div>
    </div>

    <div class="ba-card">
      <div class="ba-card-header">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/>
        </svg>
        <span>位置设置</span>
      </div>
      <p class="ba-card-desc">以屏幕左上角为原点；退出时会自动记住当前位置，留空可恢复默认。</p>
      <div class="ba-position-grid">
        <div class="ba-form-item">
          <label class="ba-label">位置 X</label>
          <n-input-number
            v-model:value="config.floatingButton.position.x"
            placeholder="自动"
            style="width: 100%;"
          />
        </div>
        <div class="ba-form-item">
          <label class="ba-label">位置 Y</label>
          <n-input-number
            v-model:value="config.floatingButton.position.y"
            placeholder="自动"
            style="width: 100%;"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
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
  gap: 14px;
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

.ba-card-desc {
  margin: 0;
  color: #5a7394;
  font-size: 13px;
  line-height: 1.65;
}

.ba-form-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
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

.ba-slider-value {
  font-size: 13px;
  font-weight: 700;
  color: #128afa;
  text-align: right;
  margin-top: -4px;
}

.ba-toggle-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.ba-toggle-label {
  font-size: 14px;
  color: #1a3a5c;
}

.ba-position-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 14px;
}
</style>
