<template>
  <div>
    <label class="inline">
      <input type="checkbox" v-model="config.pickCountDialog.defaultPlayMusic" />
      默认播放抽取背景音乐（老师如果在上课环境使用，建议先确认是否合适哦）
    </label>
    <label class="inline">
      <input type="checkbox" v-model="config.pickResultDialog.defaultPlayGachaSound" />
      默认播放抽取音效（上课时也请留意当前环境哦）
    </label>
    <label>
      抽取音效音量（0.0 - 1.0）
      <input type="number" v-model.number="config.pickResultDialog.gachaSoundVolume" min="0" max="1" step="0.05" required />
    </label>
    <label>
      背景变暗程度（0-100）
      <input type="number" v-model.number="config.pickCountDialog.backgroundDarknessPercent" min="0" max="100" required />
    </label>
    <label>
      默认抽取人数（{{ MIN_PICK_COUNT }}-{{ MAX_PICK_COUNT }}）
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
