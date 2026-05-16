<template>
  <div class="student-manager">
    <p class="desc">老师可以在这里整理当前名单和抽取权重哦。默认权重是 1.0，数值越高，就越容易被抽到。</p>
    <label class="inline">
      <input type="checkbox" v-model="config.allowRepeatDraw" />
      允许重复抽取（加权随机）
    </label>
    <div class="student-list table-wrapper">
      <div v-if="config.studentList.length === 0" class="empty-tips-text">这里还没有名单呢，老师先去“导入名单”填写一下吧。</div>
      <div v-if="config.studentList.length === 0" class="empty-tips-arona">
        <img src="/image/Arona_Empty.webp" alt="Arona Empty" class="empty-tips-arona-img" />
      </div>
      <table class="student-table" v-else>
        <thead>
          <tr>
            <th class="col-name">姓名</th>
            <th class="col-weight">权重（0.0 - 2.0）</th>
            <th class="col-action">移除</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(student, index) in config.studentList" :key="`${student.name}-${index}`">
            <td class="col-name">{{ student.name }}</td>
            <td class="col-weight">
              <input
                type="range"
                class="weight-slider"
                v-model.number="student.weight"
                min="0"
                max="2"
                step="0.1"
                @change="$emit('sync-list-to-text')"
              />
              <span class="weight-val">{{ Number(student.weight).toFixed(1) }}</span>
            </td>
            <td class="col-action">
              <button type="button" class="del-svg-btn" @click="$emit('remove-student', index)" title="移除">
                <svg viewBox="0 0 24 24" width="18" height="18" stroke="currentColor" stroke-width="2.5" fill="none" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="config.studentList.length > 0" class="student-actions">
      <button type="button" class="reset-btn" @click="$emit('reset-weights')">将所有权重重置为 1.0</button>
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

defineEmits(['sync-list-to-text', 'remove-student', 'reset-weights'])
</script>
