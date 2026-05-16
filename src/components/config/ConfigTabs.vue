<template>
  <div class="tabs">
    <button
      v-for="tab in tabs"
      :key="tab.key"
      type="button"
      class="tab-btn"
      :class="{ active: activeTab === tab.key }"
      @click="$emit('switch-tab', tab.key)"
    >
      {{ tab.label }}
    </button>
  </div>
</template>

<script setup>
defineProps({
  activeTab: {
    type: String,
    required: true
  }
})

defineEmits(['switch-tab'])

const tabs = [
  { key: 'list', label: '导入名单' },
  { key: 'students', label: '管理名单' },
  { key: 'floating', label: '悬浮按钮' },
  { key: 'pickCount', label: '抽取效果' },
  { key: 'web', label: '系统设置' }
]
</script>
