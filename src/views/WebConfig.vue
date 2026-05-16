<template>
  <n-config-provider :theme-overrides="baTheme">
    <main class="ba-page">
      <!-- Cross-hatch background decorations -->
      <div class="ba-bg-grid"></div>
      <div class="ba-bg-circle ba-bg-circle-1"></div>
      <div class="ba-bg-circle ba-bg-circle-2"></div>

      <div class="ba-shell">
        <!-- Sidebar -->
        <aside class="ba-sidebar">
          <div class="ba-sidebar-brand">
            <div class="ba-logo">
              <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
              </svg>
            </div>
            <div class="ba-brand-text">
              <h1>KVRandom</h1>
              <p>设置面板</p>
            </div>
          </div>

          <ConfigTabs :active-tab="activeTab" @switch-tab="switchTab" />

          <div class="ba-sidebar-footer">
            <button type="button" class="ba-save-btn" @click="saveConfig">
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                <polyline points="17 21 17 13 7 13 7 21"/>
                <polyline points="7 3 7 8 15 8"/>
              </svg>
              <span>保存设置</span>
            </button>
          </div>
        </aside>

        <!-- Main content area -->
        <div class="ba-main">
          <!-- Content header -->
          <div class="ba-content-header">
            <h2 class="ba-content-title">{{ currentTabTitle }}</h2>
            <p class="ba-content-hint">{{ currentTabHint }}</p>
          </div>

          <!-- Scrollable content -->
          <form id="config-form" class="ba-content-body" @submit.prevent="saveConfig">
            <div class="ba-tab-content" v-if="activeTab === 'list'" key="list">
              <StudentImportPanel
                v-model:raw-list-text="rawListText"
                :student-count="config.studentList.length"
                @schedule-sync="scheduleTextSync"
                @import-file="handleFileImport"
              />
            </div>

            <div class="ba-tab-content" v-else-if="activeTab === 'students'" key="students">
              <StudentManagerPanel
                :config="config"
                @sync-list-to-text="syncListToText"
                @remove-student="removeStudent"
                @reset-weights="resetWeights"
              />
            </div>

            <div class="ba-tab-content" v-else-if="activeTab === 'floating'" key="floating">
              <FloatingSettingsPanel :config="config" />
            </div>

            <div class="ba-tab-content" v-else-if="activeTab === 'pickCount'" key="pickCount">
              <PickSettingsPanel :config="config" />
            </div>

            <div class="ba-tab-content" v-else-if="activeTab === 'web'" key="web">
              <SystemSettingsPanel
                :config="config"
                :update-state="updateState"
                @request-admin-elevation="requestAdminElevation"
                @create-admin-startup-task="createAdminStartupTask"
                @check-update="checkUpdate"
              />
            </div>
          </form>

          <!-- Collapsible log panel -->
          <RuntimeLogPanel
            :logs="logs"
            :is-debug-mode="isDebugMode"
            :is-admin="isAdmin"
            :app-version="appVersion"
          />
        </div>
      </div>
    </main>
  </n-config-provider>
</template>

<script setup>
import { computed, onBeforeUnmount, onMounted } from 'vue'
import ConfigTabs from '../components/config/ConfigTabs.vue'
import FloatingSettingsPanel from '../components/config/FloatingSettingsPanel.vue'
import PickSettingsPanel from '../components/config/PickSettingsPanel.vue'
import RuntimeLogPanel from '../components/config/RuntimeLogPanel.vue'
import StudentImportPanel from '../components/config/StudentImportPanel.vue'
import StudentManagerPanel from '../components/config/StudentManagerPanel.vue'
import SystemSettingsPanel from '../components/config/SystemSettingsPanel.vue'
import { appApi } from '../tauriApi'
import { useAppConfig } from '../composables/useAppConfig'
import { useConfigTabs } from '../composables/useConfigTabs'
import { useLogStream } from '../composables/useLogStream'
import { useStudentListSync } from '../composables/useStudentListSync'
import { useUpdateCheck } from '../composables/useUpdateCheck'

const { activeTab, switchTab } = useConfigTabs()
const { logs, addLog, startLogStream, stopLogStream } = useLogStream(appApi)
const {
  config,
  isDebugMode,
  isAdmin,
  appVersion,
  fetchConfig,
  fetchAppInfo,
  saveConfig: saveCurrentConfig,
  requestAdminElevation,
  createAdminStartupTask
} = useAppConfig(appApi, addLog)

const {
  rawListText,
  syncTextToList,
  scheduleTextSync,
  syncListToText,
  removeStudent,
  resetWeights,
  handleFileImport,
  stopTextSync
} = useStudentListSync(appApi, config, addLog)

const { updateState, checkUpdate } = useUpdateCheck(appApi, addLog)

const saveConfig = () => saveCurrentConfig(syncTextToList)

const tabTitles = {
  list: '导入名单',
  students: '名单一览',
  floating: '悬浮按钮',
  pickCount: '抽选演出',
  web: '系统 & 更新'
}

const tabHints = {
  list: '老师～把名单交给我就好啦！粘贴文字或者导入文件都可以哦',
  students: '这里可以看到所有人的名字和权重哦，想调整的话尽管来～',
  floating: '悬浮按钮的大小、透明度、位置……老师想怎么摆都行！',
  pickCount: '音效和动画的设置都在这里～让抽选的瞬间更有仪式感吧！',
  web: '这里是比较进阶的设置了，一般保持默认就好……不过老师想改的话我也不拦着啦～'
}

const currentTabTitle = computed(() => tabTitles[activeTab.value] || '设置')
const currentTabHint = computed(() => tabHints[activeTab.value] || '老师，这里可以调整各项设置哦～')

// Blue Archive theme overrides for Naive UI
const baTheme = {
  common: {
    primaryColor: '#128AFA',
    primaryColorHover: '#3EA8FF',
    primaryColorPressed: '#0068DF',
    primaryColorSuppl: '#128AFA',
    infoColor: '#128AFA',
    infoColorHover: '#3EA8FF',
    infoColorPressed: '#0068DF',
    successColor: '#36B37E',
    successColorHover: '#57D9A3',
    successColorPressed: '#2D9F6F',
    warningColor: '#F3B900',
    warningColorHover: '#FFD84D',
    warningColorPressed: '#D4A200',
    errorColor: '#E05454',
    errorColorHover: '#F07070',
    errorColorPressed: '#C24040',
    borderRadius: '8px',
    borderRadiusSmall: '6px',
    fontFamily: '"Segoe UI Variable", "Microsoft YaHei UI", "PingFang SC", system-ui, sans-serif',
    fontSize: '14px'
  },
  Button: {
    borderRadiusMedium: '10px',
    borderRadiusSmall: '8px',
    fontWeightStrong: '700'
  },
  Switch: {
    railColorActive: '#128AFA'
  },
  Slider: {
    fillColor: '#128AFA',
    fillColorHover: '#3EA8FF',
    handleColor: '#128AFA'
  },
  Input: {
    borderRadius: '8px',
    borderHover: '1px solid #128AFA',
    borderFocus: '1px solid #128AFA',
    boxShadowFocus: '0 0 0 3px rgba(18, 138, 250, 0.12)'
  },
  Tag: {
    borderRadius: '20px'
  }
}

onMounted(() => {
  fetchConfig(rawListText)
  startLogStream()
  fetchAppInfo()
})

onBeforeUnmount(() => {
  stopTextSync()
  stopLogStream()
})
</script>

<style>
/* ============================================
   Blue Archive Settings Page — Global Styles
   ============================================ */

:root {
  --ba-blue: #128AFA;
  --ba-blue-hover: #3EA8FF;
  --ba-blue-strong: #0068DF;
  --ba-blue-soft: #E8F4FF;
  --ba-yellow: #FFD84D;
  --ba-yellow-strong: #F3B900;
  --ba-ink: #1A3A5C;
  --ba-muted: #5A7394;
  --ba-sidebar-w: 220px;
}

* {
  box-sizing: border-box;
}

html, body {
  height: 100%;
  margin: 0;
  overflow: hidden;
}

/* ---- Page shell ---- */
.ba-page {
  position: relative;
  height: 100vh;
  font-family: "Segoe UI Variable", "Microsoft YaHei UI", "PingFang SC", system-ui, sans-serif;
  color: var(--ba-ink);
  overflow: hidden;
  background: linear-gradient(160deg, #f0f7ff 0%, #e6f1ff 40%, #f5f9ff 100%);
}

/* ---- Decorative background ---- */
.ba-bg-grid {
  position: absolute;
  inset: 0;
  background:
    linear-gradient(90deg, rgba(18, 138, 250, 0.04) 1px, transparent 1px),
    linear-gradient(0deg, rgba(18, 138, 250, 0.04) 1px, transparent 1px);
  background-size: 32px 32px;
  pointer-events: none;
  z-index: 0;
}

.ba-bg-circle {
  position: absolute;
  border-radius: 50%;
  pointer-events: none;
  z-index: 0;
}

.ba-bg-circle-1 {
  width: 400px;
  height: 400px;
  top: -120px;
  right: -80px;
  background: radial-gradient(circle, rgba(18, 138, 250, 0.06) 0%, transparent 70%);
}

.ba-bg-circle-2 {
  width: 300px;
  height: 300px;
  bottom: -60px;
  left: -40px;
  background: radial-gradient(circle, rgba(243, 185, 0, 0.05) 0%, transparent 70%);
}

/* ---- Main shell grid ---- */
.ba-shell {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: var(--ba-sidebar-w) 1fr;
  height: 100vh;
  gap: 0;
}

/* ============================================
   Sidebar
   ============================================ */
.ba-sidebar {
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border-right: 1px solid rgba(18, 138, 250, 0.08);
  padding: 20px 12px;
  gap: 4px;
}

.ba-sidebar-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 4px 8px 18px;
  border-bottom: 1px solid rgba(18, 138, 250, 0.08);
  margin-bottom: 12px;
}

.ba-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 38px;
  height: 38px;
  border-radius: 10px;
  background: linear-gradient(135deg, #128AFA 0%, #3EA8FF 100%);
  color: #ffffff;
  flex-shrink: 0;
  box-shadow: 0 4px 12px rgba(18, 138, 250, 0.25);
}

.ba-brand-text h1 {
  margin: 0;
  font-size: 17px;
  font-weight: 800;
  color: #0c2d4f;
  line-height: 1.2;
  letter-spacing: -0.02em;
}

.ba-brand-text p {
  margin: 0;
  font-size: 11px;
  color: #8ca3bf;
  font-weight: 500;
}

.ba-sidebar-footer {
  margin-top: auto;
  padding-top: 12px;
  border-top: 1px solid rgba(18, 138, 250, 0.08);
}

/* ---- Save button ---- */
.ba-save-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 12px 16px;
  border: none;
  border-radius: 10px;
  background: linear-gradient(135deg, #FFE066 0%, var(--ba-yellow) 40%, var(--ba-yellow-strong) 100%);
  color: #5b4100;
  font-size: 14px;
  font-weight: 700;
  cursor: pointer;
  font-family: inherit;
  box-shadow: 0 4px 14px rgba(243, 185, 0, 0.25);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.ba-save-btn::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(90deg, transparent 0%, rgba(255, 255, 255, 0.3) 50%, transparent 100%);
  transform: translateX(-100%);
  transition: transform 0.5s;
}

.ba-save-btn:hover {
  box-shadow: 0 6px 20px rgba(243, 185, 0, 0.35);
  transform: translateY(-1px);
}

.ba-save-btn:hover::after {
  transform: translateX(100%);
}

.ba-save-btn:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(243, 185, 0, 0.2);
}

/* ============================================
   Main content area
   ============================================ */
.ba-main {
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.ba-content-header {
  padding: 22px 28px 16px;
  border-bottom: 1px solid rgba(18, 138, 250, 0.06);
  background: rgba(255, 255, 255, 0.4);
  backdrop-filter: blur(8px);
  flex-shrink: 0;
}

.ba-content-title {
  margin: 0;
  font-size: 22px;
  font-weight: 800;
  color: #0c2d4f;
  letter-spacing: -0.01em;
  position: relative;
  display: inline-block;
}

.ba-content-title::after {
  content: "";
  position: absolute;
  left: 0;
  bottom: -4px;
  width: 100%;
  height: 3px;
  border-radius: 2px;
  background: linear-gradient(90deg, var(--ba-blue) 0%, var(--ba-blue-hover) 100%);
  opacity: 0.6;
}

.ba-content-hint {
  margin: 8px 0 0;
  font-size: 13px;
  color: var(--ba-muted);
}

/* Scrollable form body */
.ba-content-body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 20px 28px;
  scrollbar-width: thin;
  scrollbar-color: #c4ddf5 transparent;
}

.ba-content-body::-webkit-scrollbar {
  width: 6px;
}

.ba-content-body::-webkit-scrollbar-track {
  background: transparent;
}

.ba-content-body::-webkit-scrollbar-thumb {
  background: #c4ddf5;
  border-radius: 3px;
}

.ba-content-body::-webkit-scrollbar-thumb:hover {
  background: #9bcfff;
}

.ba-tab-content {
  animation: ba-fade-in 0.3s ease;
}

@keyframes ba-fade-in {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* ============================================
   Responsive
   ============================================ */
@media (max-width: 768px) {
  :root {
    --ba-sidebar-w: 64px;
  }

  .ba-brand-text,
  .ba-save-btn span {
    display: none;
  }

  .ba-sidebar-brand {
    justify-content: center;
    padding-bottom: 12px;
  }

  .ba-save-btn {
    padding: 12px;
  }

  .ba-content-header {
    padding: 16px 18px 12px;
  }

  .ba-content-body {
    padding: 14px 18px;
  }
}
</style>
