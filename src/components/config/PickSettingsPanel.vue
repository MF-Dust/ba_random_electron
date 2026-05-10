<template>
  <div>
    <label class="inline">
      <input type="checkbox" v-model="config.pickCountDialog.defaultPlayMusic" />
      是否默认播放抽取背景音乐（注意！教学环境下可能并不适宜）
    </label>
    <label class="inline">
      <input type="checkbox" v-model="config.pickResultDialog.defaultPlayGachaSound" />
      抽取动画默认播放抽取音效（注意！教学环境下可能并不适宜）
    </label>
    <label>
      抽取音效响度（0.0 - 1.0）
      <input type="number" v-model.number="config.pickResultDialog.gachaSoundVolume" min="0" max="1" step="0.05" required />
    </label>
    <label>
      背景变暗程度，百分数值（0-100）
      <input type="number" v-model.number="config.pickCountDialog.backgroundDarknessPercent" min="0" max="100" required />
    </label>
    <label>
      默认人数（{{ MIN_PICK_COUNT }}-{{ MAX_PICK_COUNT }}）
      <input
        type="number"
        v-model.number="config.pickCountDialog.defaultCount"
        :min="MIN_PICK_COUNT"
        :max="MAX_PICK_COUNT"
        required
      />
    </label>
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
