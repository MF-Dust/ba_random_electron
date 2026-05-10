<template>
  <div>
    <p class="desc">老师注意！这些配置涉及程序基本运行。正常情况下，老师是不需要调整这里的配置的哦~</p>
    <div class="admin-block always-top-block">
      <p class="admin-title">管理员置顶增强（Windows）</p>
      <label class="inline">
        <input type="checkbox" v-model="config.webConfig.adminTopmostEnabled" />
        启用启动时申请管理员权限置顶
      </label>
      <p class="admin-hint">开启后程序启动会弹出 UAC 提示，以提升悬浮按钮的置顶能力。</p>
      <button type="button" class="admin-btn" @click="$emit('request-admin-elevation')">管理员身份重启</button>
    </div>

    <div class="admin-block auto-start-block">
      <p class="admin-title">开机计划任务（管理员运行）</p>
      <label>
        可执行文件路径（exe）
        <input type="text" v-model="config.webConfig.adminAutoStartPath" placeholder="例如：C:\\Program Files\\KVRandom\\kvrandom.exe" />
      </label>
      <label>
        任务名称
        <input type="text" v-model="config.webConfig.adminAutoStartTaskName" />
      </label>
      <p class="admin-hint">点击按钮后会创建/更新计划任务，登录时以管理员权限启动。</p>
      <button type="button" class="admin-btn" @click="$emit('create-admin-startup-task')">创建/更新计划任务</button>
    </div>

    <div class="admin-block update-block">
      <p class="admin-title">检查更新</p>
      <div class="update-row">
        <button type="button" class="update-btn" :disabled="updateState.loading" @click="$emit('check-update')">
          {{ updateState.loading ? '检查中...' : '检查更新' }}
        </button>
        <span class="update-status" :class="`status-${updateState.status}`">{{ updateState.title }}</span>
      </div>
      <p v-if="updateState.detail" class="update-detail">{{ updateState.detail }}</p>
      <div v-if="updateState.commitUrl || updateState.releaseUrl" class="update-links">
        <a v-if="updateState.commitUrl" :href="updateState.commitUrl" target="_blank" rel="noopener">查看提交</a>
        <a v-if="updateState.releaseUrl" :href="updateState.releaseUrl" target="_blank" rel="noopener">查看发布页</a>
      </div>
    </div>

    <div class="admin-block trademark-block">
      <p class="admin-title">商标与版权声明</p>
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
