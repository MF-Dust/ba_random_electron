<template>
  <div class="ba-log-panel" :class="{ collapsed: !expanded }">
    <button type="button" class="ba-log-header" @click="expanded = !expanded">
      <div class="ba-log-title-row">
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="4 17 10 11 4 5"/><line x1="12" y1="19" x2="20" y2="19"/>
        </svg>
        <span class="ba-log-title">运行记录</span>
        <n-tag v-if="isDebugMode" type="info" size="small" round>Dev</n-tag>
        <n-tag v-if="isAdmin" type="warning" size="small" round>管理员</n-tag>
        <n-tag size="small" round :bordered="false">v{{ appVersion }}</n-tag>
      </div>
      <svg class="ba-log-chevron" viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="6 9 12 15 18 9"/>
      </svg>
    </button>
    <transition name="ba-log-slide">
      <div v-if="expanded" class="ba-log-body">
        <div class="ba-log-list" role="log" aria-live="polite">
          <div v-if="logs.length === 0" class="ba-log-empty">
            <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="opacity:0.4">
              <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
            <span>暂时还没有新的记录哦</span>
          </div>
          <div v-for="item in logs" :key="item.id" class="ba-log-item" :class="`ba-log-${item.level}`">
            <span class="ba-log-time">{{ item.time }}</span>
            <span class="ba-log-text">{{ item.text }}</span>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { NTag } from 'naive-ui'

defineProps({
  logs: {
    type: Array,
    required: true
  },
  isDebugMode: {
    type: Boolean,
    required: true
  },
  isAdmin: {
    type: Boolean,
    required: true
  },
  appVersion: {
    type: String,
    required: true
  }
})

const expanded = ref(false)
</script>

<style scoped>
.ba-log-panel {
  border-top: 1px solid rgba(18, 138, 250, 0.12);
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(8px);
  flex-shrink: 0;
}

.ba-log-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 12px 20px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  transition: background 0.2s;
}

.ba-log-header:hover {
  background: rgba(18, 138, 250, 0.04);
}

.ba-log-title-row {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #3d5a80;
}

.ba-log-title {
  font-size: 14px;
  font-weight: 700;
  color: #1a3a5c;
}

.ba-log-chevron {
  color: #8ca3bf;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.ba-log-panel:not(.collapsed) .ba-log-chevron {
  transform: rotate(180deg);
}

.ba-log-body {
  max-height: 200px;
  overflow: hidden;
}

.ba-log-list {
  display: flex;
  flex-direction: column-reverse;
  gap: 6px;
  padding: 0 16px 14px;
  max-height: 186px;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #b8d4f0 transparent;
}

.ba-log-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 16px;
  color: #8ca3bf;
  font-size: 13px;
  background: rgba(18, 138, 250, 0.03);
  border: 1px dashed rgba(18, 138, 250, 0.15);
  border-radius: 8px;
}

.ba-log-item {
  display: grid;
  grid-template-columns: 60px 1fr;
  gap: 10px;
  padding: 8px 12px;
  border-radius: 6px;
  background: #ffffff;
  border: 1px solid #e8f0f8;
  font-size: 12px;
  border-left: 3px solid #d0dde8;
  transition: background 0.15s;
}

.ba-log-item.ba-log-info {
  border-left-color: #128afa;
}

.ba-log-item.ba-log-success {
  border-left-color: #36b37e;
  background: #f6fff9;
}

.ba-log-item.ba-log-error {
  border-left-color: #e05454;
  background: #fff8f8;
}

.ba-log-time {
  font-variant-numeric: tabular-nums;
  color: #8ca3bf;
  font-size: 11px;
}

.ba-log-text {
  color: #1a3a5c;
  word-break: break-word;
  overflow-wrap: anywhere;
}

/* transition */
.ba-log-slide-enter-active,
.ba-log-slide-leave-active {
  transition: max-height 0.35s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.25s;
}

.ba-log-slide-enter-from,
.ba-log-slide-leave-to {
  max-height: 0;
  opacity: 0;
}

.ba-log-slide-enter-to,
.ba-log-slide-leave-from {
  max-height: 200px;
  opacity: 1;
}
</style>
