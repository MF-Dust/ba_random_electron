<template>
  <div>
    <p class="desc">老师请留意，这里的设置会影响程序的基础运行。一般情况下保持默认就可以啦。</p>
    <div class="admin-block always-top-block">
      <p class="admin-title">管理员置顶增强（Windows）</p>
      <label class="inline">
        <input type="checkbox" v-model="config.webConfig.adminTopmostEnabled" />
        启动时申请管理员权限，以增强置顶效果
      </label>
      <p class="admin-hint">开启后，程序启动时会弹出 UAC 提示，用来提升悬浮按钮的置顶能力。</p>
      <button type="button" class="admin-btn" @click="$emit('request-admin-elevation')">以管理员身份重新启动</button>
    </div>

    <div class="admin-block auto-start-block">
      <p class="admin-title">开机启动任务（管理员运行）</p>
      <label>
        程序路径（exe）
        <input type="text" v-model="config.webConfig.adminAutoStartPath" placeholder="例如：C:\\Program Files\\KVRandom\\kvrandom.exe" />
      </label>
      <label>
        任务名称
        <input type="text" v-model="config.webConfig.adminAutoStartTaskName" />
      </label>
      <p class="admin-hint">点击后会创建或更新计划任务，登录系统时将以管理员权限启动。</p>
      <button type="button" class="admin-btn" @click="$emit('create-admin-startup-task')">创建或更新计划任务</button>
    </div>

    <div class="admin-block update-block">
      <p class="admin-title">检查更新</p>
      <div class="update-row">
        <button type="button" class="update-btn" :disabled="updateState.loading" @click="$emit('check-update')">
          {{ updateState.loading ? '正在检查中...' : '检查更新' }}
        </button>
        <span class="update-status" :class="`status-${updateState.status}`">{{ updateState.title }}</span>
      </div>
      <p v-if="updateState.detail" class="update-detail">{{ updateState.detail }}</p>
      <div v-if="updateState.commitUrl || updateState.releaseUrl" class="update-links">
        <a v-if="updateState.commitUrl" :href="updateState.commitUrl" target="_blank" rel="noopener">查看这次提交</a>
        <a v-if="updateState.releaseUrl" :href="updateState.releaseUrl" target="_blank" rel="noopener">查看发布页面</a>
      </div>
    </div>

    <div class="admin-block trademark-block">
      <p class="admin-title">商标与版权说明</p>
      <p class="admin-hint">“蔚蓝档案”是上海星啸网络科技有限公司的注册商标，版权所有。</p>
      <p class="admin-hint">「ブルーアーカイブ」は株式会社Yostarの登録商標です。著作権はすべて保有されています。</p>
      <p class="admin-hint">"Blue Archive" is a registered trademark of NEXON Korea Corp. & NEXON GAMES Co., Ltd. All rights reserved.</p>
    </div>
  </div>
</template>

<script setup>
defineProps({
  config: {
    type: Object,
    required: true
  },
  updateState: {
    type: Object,
    required: true
  }
})

defineEmits(['request-admin-elevation', 'create-admin-startup-task', 'check-update'])
</script>
