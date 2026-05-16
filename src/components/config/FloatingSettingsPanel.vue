<template>
  <div>
    <label>
      悬浮按钮大小（以 50×50 px 为 100，范围 0-1000）
      <input type="number" v-model.number="config.floatingButton.sizePercent" min="0" max="1000" required />
    </label>
    <label>
      悬浮按钮透明度（范围 0-100）
      <input type="number" v-model.number="config.floatingButton.transparencyPercent" min="0" max="100" required />
    </label>
    <label class="inline">
      <input type="checkbox" v-model="config.floatingButton.alwaysOnTop" />
      始终置顶显示
    </label>
    <div class="row">
      <label>
        位置 X（以屏幕左上角为原点；退出时会自动记住当前位置，留空可恢复默认）
        <input type="number" v-model.number="config.floatingButton.position.x" />
      </label>
      <label>
        位置 Y（以屏幕左上角为原点；退出时会自动记住当前位置，留空可恢复默认）
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
