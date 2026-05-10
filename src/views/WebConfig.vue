<template>
  <main class="page">
    <div class="layout">
      <section class="panel panel-left">
        <div class="header">
          <h1>蔚蓝点名 Web配置页</h1>
          <p class="hint">老师可以在这里配置 蔚蓝点名 的各项功能哦！</p>
        </div>

        <ConfigTabs :active-tab="activeTab" @switch-tab="switchTab" />

        <form id="config-form" @submit.prevent="saveConfig">
          <div class="tab-container">
            <transition :name="transitionName" mode="out-in">
              <div class="tab-content" v-if="activeTab === 'list'" key="list">
                <StudentImportPanel
                  v-model:raw-list-text="rawListText"
                  :student-count="config.studentList.length"
                  @schedule-sync="scheduleTextSync"
                  @import-file="handleFileImport"
                />
              </div>

              <div class="tab-content" v-else-if="activeTab === 'students'" key="students">
                <StudentManagerPanel
                  :config="config"
                  @sync-list-to-text="syncListToText"
                  @remove-student="removeStudent"
                  @reset-weights="resetWeights"
                />
              </div>

              <div class="tab-content" v-else-if="activeTab === 'floating'" key="floating">
                <FloatingSettingsPanel :config="config" />
              </div>

              <div class="tab-content" v-else-if="activeTab === 'pickCount'" key="pickCount">
                <PickSettingsPanel :config="config" />
              </div>

              <div class="tab-content" v-else-if="activeTab === 'web'" key="web">
                <SystemSettingsPanel
                  :config="config"
                  :update-state="updateState"
                  @request-admin-elevation="requestAdminElevation"
                  @create-admin-startup-task="createAdminStartupTask"
                  @check-update="checkUpdate"
                />
              </div>
            </transition>
          </div>

          <button type="submit" class="save-btn">保存配置</button>
        </form>
      </section>

      <RuntimeLogPanel
        :logs="logs"
        :is-debug-mode="isDebugMode"
        :is-admin="isAdmin"
        :app-version="appVersion"
      />
    </div>
  </main>
</template>

<script setup>
import { onBeforeUnmount, onMounted } from 'vue'
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

const { activeTab, transitionName, switchTab } = useConfigTabs()
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

const saveConfig = () => saveCurrentConfig(syncTextToList, rawListText)

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
.page * {
  box-sizing: border-box;
}

html,
body {
  height: 100%;
  margin: 0;
  overflow: hidden;
}

.page {
  height: 100vh;
  padding: 28px;
  display: flex;
  align-items: stretch;
  justify-content: center;
  font-family: "Segoe UI Variable", "Microsoft YaHei UI", "PingFang SC", sans-serif;
  background:
    radial-gradient(1200px 800px at 20% 10%, rgba(148, 199, 255, 0.28), transparent 60%),
    radial-gradient(900px 600px at 80% 0%, rgba(167, 222, 255, 0.22), transparent 55%),
    #eef3fb;
  color: #0f1f3b;
  overflow: hidden;
}

.layout {
  width: 100%;
  min-height: 0;
  display: grid;
  grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);
  grid-template-rows: 1fr;
  gap: 20px;
  height: 100%;
}

.page .panel {
  background: linear-gradient(140deg, rgba(255, 255, 255, 0.92), rgba(245, 248, 255, 0.88));
  border: 1px solid rgba(142, 175, 210, 0.4);
  border-radius: 16px;
  box-shadow: 0 18px 38px rgba(12, 28, 59, 0.12);
  padding: 22px 24px;
  backdrop-filter: blur(18px);
}

.page .panel-left {
  min-height: 0;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.page .panel-right {
  display: flex;
  flex-direction: column;
  min-height: 0;
  min-width: 0;
}

.page h1 {
  margin: 0;
  font-size: 30px;
  letter-spacing: 1px;
  text-align: left;
}
.header {
  text-align: left;
}

.hint {
  margin-top: 8px;
  color: #355985;
  text-align: left;
}

.page .tabs {
  display: flex;
  margin: 20px 0 18px;
  border-bottom: 1px solid rgba(120, 148, 185, 0.4);
  background: rgba(255, 255, 255, 0.7);
  border-radius: 12px;
  padding: 4px;
  gap: 6px;
}

.page .tab-btn {
  flex: 1;
  background: transparent;
  border: none;
  font-size: 14px;
  font-weight: 600;
  color: #3a4c6b;
  padding: 10px 0;
  cursor: pointer;
  transition: all 0.2s ease;
  border-radius: 10px;
}

.page .tab-btn:hover {
  background: rgba(220, 232, 249, 0.7);
}

.page .tab-btn.active {
  color: #ffffff;
  background: rgba(7, 105, 241, 0.92);
  box-shadow: 0 8px 18px rgba(16, 32, 59, 0.12);
}

.page #config-form {
  display: flex;
  flex-direction: column;
  flex: 1;
  height: 0;
  min-height: 0;
}

.tab-content {
  padding: 10px 0;
  height: 100%;
}

.tab-container {
  flex: 1 1 0;
  min-height: 0;
  height: 0;
  overflow-y: scroll; 
  padding-right: 6px;
  position: relative;
}

.list-manager {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.page .desc {
  margin: 0;
  font-size: 14px;
  color: #4a6c94;
}

.list-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.upload-btn {
  display: inline-flex;
  align-items: center;
  background-color: #e5f1ff;
  color: #1a4d8c;
  padding: 8px 16px;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  border: 1px solid #aac2e0;
  transition: all 0.2s ease;
  margin: 0 !important;
}

.upload-btn:hover {
  background-color: #d0e6ff;
}

.count-badge {
  font-size: 14px;
  font-weight: bold;
  color: #2c69b2;
  background: #eef5fc;
  padding: 4px 10px;
  border-radius: 20px;
}

.list-textarea {
  width: 100%;
  height: 200px;
  min-height: 120px;
  border: 1px solid rgba(127, 157, 193, 0.55);
  border-radius: 12px;
  padding: 12px;
  font-size: 15px;
  resize: vertical;
  line-height: 1.6;
  font-family: inherit;
  background: rgba(255, 255, 255, 0.9);
}

.list-textarea:focus {
  outline: none;
  border-color: #5a89c8;
  box-shadow: 0 0 0 3px rgba(90, 137, 200, 0.2);
}

.page label {
  display: block;
  margin: 10px 0;
  font-size: 14px;
  color: #2a4365;
}

.page .inline {
  display: flex;
  align-items: center;
  gap: 8px;
}

.page input[type="number"],
.page input[type="text"],
.page textarea,
.page select {
  width: 100%;
  margin-top: 6px;
  border: 1px solid rgba(122, 151, 190, 0.55);
  border-radius: 14px;
  padding: 10px 12px;
  font-size: 15px;
  background: rgba(255, 255, 255, 0.92);
  color: #102743;
  transition: border-color 160ms ease, box-shadow 160ms ease, transform 160ms ease, background 160ms ease;
}

.page input[type="number"]:focus,
.page input[type="text"]:focus,
.page textarea:focus,
.page select:focus {
  outline: none;
  border-color: rgba(45, 110, 210, 0.8);
  box-shadow: 0 0 0 4px rgba(74, 130, 220, 0.18);
  background: #ffffff;
}

.page input[type="number"]:disabled,
.page input[type="text"]:disabled,
.page textarea:disabled,
.page select:disabled {
  opacity: 0.7;
  cursor: not-allowed;
  background: rgba(235, 241, 249, 0.7);
}

.page input::placeholder,
.page textarea::placeholder {
  color: rgba(90, 113, 145, 0.7);
}

.page input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: #2a6bff;
  border-radius: 5px;
}

.page .row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.save-btn {
  margin-top: 20px;
  width: 100%;
  border: 0;
  border-radius: 12px;
  padding: 13px 14px;
  color: #0f1f3b;
  font-size: 16px;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(135deg, #f1f601, #f3b703);
  box-shadow: 0 10px 20px rgba(32, 63, 115, 0.2);
  transition: background 0.2s ease;
  
}

.save-btn:hover {
  background: linear-gradient(135deg, #bdd6ff, #eef3ff);
  
}

.update-header {
  margin-bottom: 5px;
  font-size: 14px;
}
.update-card {
  margin-top: 0px;
  border: 1px solid rgba(134, 162, 200, 0.5);
  border-radius: 12px;
  padding: 14px 16px;
  background: rgba(255, 255, 255, 0.7);
  box-shadow: 0 10px 24px rgba(18, 36, 69, 0.08);
}

.update-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.update-btn {
  border: 0;
  margin-top: 3px;
  border-radius: 10px;
  padding: 8px 16px;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(135deg, #5aa6ff, #2c6df5);
  color: #fff;
  box-shadow: 0 8px 16px rgba(23, 65, 134, 0.2);
}

.update-btn:disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.update-status {
  font-size: 13px;
  font-weight: 600;
  color: #3f5b7a;
}

.update-status.status-update {
  color: #1b5fd1;
}

.update-status.status-ok {
  color: #2f7d4b;
}

.update-status.status-easter {
  color: #a45b2c;
}

.update-status.status-error {
  color: #c24040;
}

.update-detail {
  border: 1px solid #75c5fe5c;
  border-radius: 8px;
  padding: 10px 12px;
  background: rgba(217, 239, 255, 0.15);
  font-size: 13px;
  color: #4a6c94;
  white-space: pre-wrap;
}

.update-links {
  margin-top: 8px;
  display: flex;
  gap: 12px;
}

.update-links a {
  color: #2f63c2;
  text-decoration: none;
  font-size: 13px;
  font-weight: 600;
}

.update-links a:hover {
  text-decoration: underline;
}

.admin-block {
  margin-top: 18px;
  padding: 14px 16px;
  border-radius: 12px;
  border: 1px solid rgba(120, 150, 190, 0.35);
  background: rgba(238, 244, 252, 0.7);
}

.admin-block.always-top-block {
  background: rgba(255, 237, 199, 0.7);
  border-color: rgba(255, 188, 44, 0.4);
}

.admin-block.update-block {
  background: rgba(250, 234, 255, 0.354);
  border-color: rgba(255, 68, 249, 0.4);
}

.admin-title {
  margin: 0 0 8px;
  font-weight: 700;
  color: #123564;
}

.admin-hint {
  margin: 6px 0 10px;
  font-size: 13px;
  color: rgba(20, 40, 70, 0.78);
}

.admin-btn {
  height: 40px;
  border-radius: 8px;
  padding: 0px 16px;
  border: 0;
  cursor: pointer;
  font-size: 14px;
  font-weight: 700;
  color: #2a4365;
  background: linear-gradient(135deg, #d2f0ff, #9bdeff);
  box-shadow: 0 10px 18px rgba(30, 52, 92, 0.18);
  transition: transform 120ms ease, filter 120ms ease, box-shadow 120ms ease;
}

.admin-btn:hover {
  filter: brightness(1.05);
}

.admin-btn:active {
  transform: translateY(1px) scale(0.985);
}

.student-manager {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.table-wrapper {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid #aac2e0;
  border-radius: 8px;
  background: #fdfdfd;
}
.empty-tips-text {
  text-align: center;
  color: #92a4ba;
  padding-bottom: 0px;
  padding-top: 40px;
}

.empty-tips-arona {
  text-align: center;
  color: #92a4ba;
  padding-bottom: 40px;
  padding-top: 10px;
}


.empty-tips-arona-img {
  width: 20%;
  opacity: 0.8;
}
.student-table {
  width: 100%;
  border-collapse: collapse;
  text-align: left;
}
.student-table th, .student-table td {
  padding: 10px 14px;
  border-bottom: 1px solid #e1e9f2;
}
.student-table th {
  background: #eef5fc;
  color: #355985;
  font-weight: 600;
  position: sticky;
  top: 0;
  z-index: 1;
}
.col-name {
  width: 40%;
  font-weight: 600;
  color: #133053;
}
.col-weight {
  width: 45%;
  white-space: nowrap;
}
.col-action {
  width: 15%;
  text-align: center;
}
.student-table th.col-action {
  text-align: center;
}
.weight-slider {
  vertical-align: middle;
  width: 120px;
}
.weight-val {
  display: inline-block;
  width: 30px;
  margin-left: 8px;
  font-size: 13px;
  color: #4a6c94;
  vertical-align: middle;
}
.del-svg-btn {
  background: none;
  border: none;
  color: #c92a2a;
  cursor: pointer;
  padding: 6px;
  border-radius: 50%;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;
}
.del-svg-btn:hover {
  background: #ffeaed;
}
.reset-btn {
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  padding: 6px 12px;
  background: #e5f1ff;
  color: #1a4d8c;
}
.reset-btn:hover { background: #d0e6ff; }
.student-actions {
  display: flex;
  justify-content: flex-end;
}

.log-header {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
}

.log-title-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.log-header h2 {
  margin: 0;
  font-size: 20px;
}

.debug-badge {
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 700;
  color: #ffffff;
  background: linear-gradient(135deg, #2a6bff, #61a0ff);
  box-shadow: 0 6px 14px rgba(36, 94, 190, 0.25);
}

.admin-badge {
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 700;
  color: #3a2600;
  background: linear-gradient(135deg, #ffd36a, #ffb347);
  box-shadow: 0 6px 14px rgba(180, 120, 20, 0.25);
}

.version-badge {
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 700;
  color: #1f2a44;
  background: rgba(255, 255, 255, 0.85);
  border: 1px solid rgba(120, 148, 185, 0.5);
}

.log-list {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column-reverse;
  gap: 10px;
  padding-right: 6px;
  overflow: auto;
}

.log-item {
  display: grid;
  grid-template-columns: 58px 1fr;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(164, 186, 216, 0.4);
  box-shadow: 0 8px 18px rgba(18, 36, 69, 0.08);
  font-size: 13px;
}

.log-item.log-success {
  border-color: rgba(102, 175, 126, 0.6);
  background: rgba(227, 245, 233, 0.8);
}

.log-item.log-error {
  border-color: rgba(214, 110, 110, 0.55);
  background: rgba(255, 231, 231, 0.85);
}

.log-item.log-info {
  border-color: rgba(124, 160, 206, 0.55);
}

.log-time {
  font-variant-numeric: tabular-nums;
  color: #6b7d99;
}

.log-text {
  color: #1f2a44;
  word-break: break-word;
  overflow-wrap: anywhere;
}

.log-empty {
  padding: 12px;
  color: #7b8da8;
  text-align: center;
  background: rgba(255, 255, 255, 0.6);
  border: 1px dashed rgba(151, 177, 210, 0.5);
  border-radius: 12px;
}

.slide-left-enter-active, .slide-left-leave-active,
.slide-right-enter-active, .slide-right-leave-active {
  transition: transform 0.25s ease, opacity 0.25s ease;
}

.slide-left-enter-from {
  opacity: 0;
  transform: translateX(20px);
}
.slide-left-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

.slide-right-enter-from {
  opacity: 0;
  transform: translateX(-20px);
}
.slide-right-leave-to {
  opacity: 0;
  transform: translateX(20px);
}
</style>

<style>
:root {
  --ba-blue: #1284ff;
  --ba-blue-strong: #0068df;
  --ba-blue-soft: #e8f4ff;
  --ba-blue-line: #b7d9f8;
  --ba-yellow: #ffd84d;
  --ba-yellow-strong: #f3b900;
  --ba-ink: #10243f;
  --ba-muted: #5f7694;
  --ba-panel: #ffffff;
  --ba-surface: #f6fbff;
  --ba-line: #d7e5f3;
  --ba-shadow: 0 18px 34px rgba(36, 91, 145, 0.14);
}

.page {
  padding: 24px;
  background:
    linear-gradient(90deg, rgba(18, 132, 255, 0.055) 1px, transparent 1px),
    linear-gradient(0deg, rgba(18, 132, 255, 0.055) 1px, transparent 1px),
    linear-gradient(180deg, #f7fcff 0%, #edf7ff 52%, #f9fcff 100%);
  background-size: 28px 28px, 28px 28px, auto;
  color: var(--ba-ink);
}

.layout {
  gap: 18px;
}

.page .panel {
  background: var(--ba-panel);
  border: 1px solid var(--ba-line);
  border-top: 4px solid var(--ba-blue);
  border-radius: 12px;
  box-shadow: var(--ba-shadow);
  backdrop-filter: none;
}

.page .panel-left {
  padding: 20px 22px;
}

.page .panel-right {
  padding: 18px;
}

.header {
  position: relative;
  padding: 2px 0 14px 18px;
  border-bottom: 1px solid var(--ba-line);
}

.header::before {
  content: "";
  position: absolute;
  left: 0;
  top: 5px;
  width: 6px;
  height: 42px;
  border-radius: 999px;
  background: linear-gradient(180deg, var(--ba-blue), #76c7ff);
}

.page h1 {
  font-size: 28px;
  line-height: 1.1;
  letter-spacing: 0;
  color: #0c315f;
}

.hint {
  margin: 8px 0 0;
  color: var(--ba-muted);
  font-size: 14px;
}

.page .tabs {
  margin: 16px 0 14px;
  padding: 3px;
  gap: 4px;
  border: 1px solid var(--ba-blue-line);
  border-radius: 10px;
  background: linear-gradient(180deg, #ffffff 0%, #edf7ff 100%);
}

.page .tab-btn {
  min-height: 38px;
  border-radius: 7px;
  color: #315578;
  font-weight: 700;
  letter-spacing: 0;
}

.page .tab-btn:hover {
  color: var(--ba-blue-strong);
  background: rgba(18, 132, 255, 0.1);
}

.page .tab-btn.active {
  color: #ffffff;
  background: linear-gradient(180deg, #35a8ff 0%, var(--ba-blue-strong) 100%);
  box-shadow: 0 8px 16px rgba(0, 104, 223, 0.24);
}

.tab-container {
  padding: 12px 8px 12px 0;
  scrollbar-color: #9bcfff transparent;
}

.tab-content {
  padding: 0;
}

.list-manager,
.student-manager,
.admin-block {
  border: 1px solid var(--ba-line);
  border-radius: 10px;
  background: var(--ba-surface);
}

.list-manager,
.student-manager {
  padding: 16px;
}

.page .desc {
  color: var(--ba-muted);
  line-height: 1.65;
}

.list-actions {
  gap: 12px;
}

.upload-btn,
.reset-btn,
.admin-btn,
.update-btn {
  border-radius: 8px;
  border: 1px solid var(--ba-blue-line);
  background: linear-gradient(180deg, #ffffff 0%, var(--ba-blue-soft) 100%);
  color: #0f4d88;
  font-weight: 800;
  box-shadow: 0 6px 12px rgba(32, 96, 150, 0.12);
}

.upload-btn:hover,
.reset-btn:hover,
.admin-btn:hover,
.update-btn:hover:not(:disabled) {
  filter: none;
  border-color: #62b8ff;
  background: linear-gradient(180deg, #ffffff 0%, #d9efff 100%);
}

.upload-btn:active,
.reset-btn:active,
.admin-btn:active,
.update-btn:active:not(:disabled) {
  transform: translateY(1px);
  box-shadow: none;
}

.count-badge,
.debug-badge,
.admin-badge,
.version-badge {
  border-radius: 7px;
  border: 1px solid var(--ba-blue-line);
  background: #ffffff;
  color: #245b91;
  box-shadow: none;
}

.debug-badge {
  color: #ffffff;
  background: var(--ba-blue);
  border-color: var(--ba-blue);
}

.admin-badge {
  color: #5b4100;
  background: #fff3bd;
  border-color: #f2cf62;
}

.list-textarea,
.page input[type="number"],
.page input[type="text"],
.page textarea,
.page select {
  border-radius: 8px;
  border-color: #bdd4eb;
  background: #ffffff;
  color: var(--ba-ink);
  box-shadow: inset 0 1px 0 rgba(18, 132, 255, 0.05);
}

.list-textarea:focus,
.page input[type="number"]:focus,
.page input[type="text"]:focus,
.page textarea:focus,
.page select:focus {
  border-color: var(--ba-blue);
  box-shadow:
    0 0 0 3px rgba(18, 132, 255, 0.16),
    inset 0 1px 0 rgba(18, 132, 255, 0.05);
}

.page input[type="checkbox"] {
  accent-color: var(--ba-blue);
}

.save-btn {
  min-height: 46px;
  border-radius: 10px;
  color: #3b2b00;
  background: linear-gradient(180deg, #fff27a 0%, var(--ba-yellow) 52%, var(--ba-yellow-strong) 100%);
  border: 1px solid #e3b900;
  box-shadow:
    0 10px 18px rgba(197, 144, 0, 0.22),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
}

.save-btn:hover {
  background: linear-gradient(180deg, #fff79d 0%, #ffe060 58%, #ffc928 100%);
}

.save-btn:active {
  transform: translateY(1px);
  box-shadow: inset 0 2px 4px rgba(122, 84, 0, 0.18);
}

.table-wrapper {
  border-color: var(--ba-line);
  border-radius: 10px;
  background: #ffffff;
}

.student-table th {
  background: linear-gradient(180deg, #eaf6ff 0%, #dceeff 100%);
  color: #174a78;
  border-bottom: 1px solid var(--ba-blue-line);
}

.student-table td {
  color: var(--ba-ink);
}

.student-table tbody tr:hover {
  background: #f5fbff;
}

.weight-slider {
  accent-color: var(--ba-blue);
}

.weight-val {
  color: #1c6dae;
  font-weight: 700;
}

.del-svg-btn {
  color: #d44747;
}

.del-svg-btn:hover {
  background: #fff0f0;
}

.admin-block {
  padding: 15px 16px;
  background: #f7fbff;
}

.admin-block.always-top-block {
  background: #fff9e8;
  border-color: #ecd27d;
}

.admin-block.update-block {
  background: #f8f3ff;
  border-color: #cdb7f5;
}

.admin-title {
  color: #0d3a67;
  font-size: 15px;
}

.admin-hint {
  color: var(--ba-muted);
  line-height: 1.55;
}

.update-row {
  align-items: center;
}

.update-btn {
  color: #ffffff;
  border-color: var(--ba-blue-strong);
  background: linear-gradient(180deg, #42b2ff 0%, var(--ba-blue-strong) 100%);
}

.update-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, #62c2ff 0%, #0877ef 100%);
}

.update-detail {
  border-color: var(--ba-line);
  background: #ffffff;
  color: var(--ba-muted);
}

.update-status {
  color: var(--ba-muted);
}

.log-header {
  padding-bottom: 13px;
  border-bottom: 1px solid var(--ba-line);
}

.log-header h2 {
  color: #0c315f;
  font-size: 19px;
}

.log-list {
  padding-top: 14px;
}

.log-item {
  grid-template-columns: 64px 1fr;
  border-radius: 8px;
  border-color: var(--ba-line);
  background: #ffffff;
  box-shadow: none;
}

.log-item.log-info {
  border-left: 4px solid var(--ba-blue);
}

.log-item.log-success {
  border-left: 4px solid #3cb878;
  background: #f5fff9;
}

.log-item.log-error {
  border-left: 4px solid #e05454;
  background: #fff7f7;
}

.log-time {
  color: #8094aa;
}

.log-text {
  color: var(--ba-ink);
}

.log-empty {
  border-radius: 8px;
  border-color: var(--ba-blue-line);
  background: #f8fcff;
  color: var(--ba-muted);
}

@media (max-width: 900px) {
  .page {
    padding: 14px;
  }

  .layout {
    grid-template-columns: 1fr;
    grid-template-rows: minmax(0, 1fr) 320px;
  }

  .page .tabs {
    overflow-x: auto;
  }

  .page .tab-btn {
    min-width: 112px;
  }
}
</style>
