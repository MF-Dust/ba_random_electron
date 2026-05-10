<template>
  <div>
    <label>
      按钮大小，为百分数值，以50px*50px为100（范围:0-1000）
      <input type="number" v-model.number="config.floatingButton.sizePercent" min="0" max="1000" required />
    </label>
    <label>
      透明度，为百分数值（范围:0-100）
      <input type="number" v-model.number="config.floatingButton.transparencyPercent" min="0" max="100" required />
    </label>
    <label class="inline">
      <input type="checkbox" v-model="config.floatingButton.alwaysOnTop" />
      是否置顶
    </label>
    <div class="row">
      <label>
        位置 X（以屏幕左上角为坐标原点，自动在退出时保存当前位置，留空恢复到默认位置）
        <input type="number" v-model.number="config.floatingButton.position.x" />
      </label>
      <label>
        位置 Y（以屏幕左上角为坐标原点，自动在退出时保存当前位置，留空恢复到默认位置）
        <input type="number" v-model.number="config.floatingButton.position.y" />
      </label>
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
