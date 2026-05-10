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
  { key: 'list', label: '花名册导入' },
  { key: 'students', label: '花名册管理' },
  { key: 'floating', label: '抽取悬浮按钮' },
  { key: 'pickCount', label: '抽取动画' },
  { key: 'web', label: '系统服务' }
]
</script>
