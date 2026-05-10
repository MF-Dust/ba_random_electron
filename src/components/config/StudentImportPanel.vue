<template>
  <div class="list-manager">
    <p class="desc">老师可以手动输入名单（每行一个），或者点击下方按钮导入CSV/TXT文件自动解析！</p>
    <div class="list-actions">
      <button type="button" class="upload-btn" @click="$emit('import-file')">
        <span>📂 导入文件</span>
      </button>
      <span class="count-badge">当前导入人数：{{ studentCount }}</span>
    </div>
    <textarea
      :value="rawListText"
      class="list-textarea"
      placeholder="请输入名单，每行一个。例如：
早濑优香
小鸟游星野
空崎日奈"
      @input="$emit('update:raw-list-text', $event.target.value); $emit('schedule-sync')"
    ></textarea>
  </div>
</template>

<script setup>
defineProps({
  rawListText: {
    type: String,
    required: true
  },
  studentCount: {
    type: Number,
    required: true
  }
})

defineEmits(['update:raw-list-text', 'schedule-sync', 'import-file'])
</script>
