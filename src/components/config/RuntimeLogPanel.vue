<template>
  <aside class="panel panel-right">
    <div class="log-header">
      <div class="log-title-row">
        <h2>运行记录</h2>
        <span v-if="isDebugMode" class="debug-badge">Dev</span>
        <span v-if="isAdmin" class="admin-badge">管理员</span>
        <span class="version-badge">当前版本 {{ appVersion }}</span>
      </div>
    </div>
    <div class="log-list" role="log" aria-live="polite">
      <div v-if="logs.length === 0" class="log-empty">暂时还没有新的记录哦</div>
      <div v-for="item in logs" :key="item.id" class="log-item" :class="`log-${item.level}`">
        <span class="log-time">{{ item.time }}</span>
        <span class="log-text">{{ item.text }}</span>
      </div>
    </div>
  </aside>
</template>

<script setup>
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
</script>
