<template>
  <main class="page">
    <div class="layout">
      <section class="panel panel-left">
        <div class="header">
          <h1>KVRandom 设置</h1>
          <p class="hint">老师，欢迎回来。这里可以慢慢调整 KVRandom 的各项设置哦。</p>
        </div>

        <ConfigTabs :active-tab="activeTab" @switch-tab="switchTab" />

        <form id="config-form" @submit.prevent="saveConfig">
          <div class="tab-container">
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
          </div>

          <button type="submit" class="save-btn">保存设置</button>
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

* {
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
  padding: 24px;
  display: flex;
  align-items: stretch;
  justify-content: center;
  font-family: "Segoe UI Variable", "Microsoft YaHei UI", "PingFang SC", sans-serif;
  background:
    linear-gradient(90deg, rgba(18, 132, 255, 0.055) 1px, transparent 1px),
    linear-gradient(0deg, rgba(18, 132, 255, 0.055) 1px, transparent 1px),
    linear-gradient(180deg, #f7fcff 0%, #edf7ff 52%, #f9fcff 100%);
  background-size: 28px 28px, 28px 28px, auto;
  color: var(--ba-ink);
  overflow: hidden;
}

.layout {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);
  grid-template-rows: 1fr;
  gap: 18px;
}

.page .panel {
  background: var(--ba-panel);
  border: 1px solid var(--ba-line);
  border-top: 4px solid var(--ba-blue);
  border-radius: 12px;
  box-shadow: var(--ba-shadow);
  display: flex;
  flex-direction: column;
  min-height: 0;
  min-width: 0;
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
  text-align: left;
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
  margin: 0;
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
  display: flex;
  margin: 16px 0 14px;
  padding: 3px;
  gap: 4px;
  border: 1px solid var(--ba-blue-line);
  border-radius: 10px;
  background: linear-gradient(180deg, #ffffff 0%, #edf7ff 100%);
}

.page .tab-btn {
  flex: 1;
  min-height: 38px;
  background: transparent;
  border: none;
  border-radius: 7px;
  color: #315578;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.2s ease;
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

.page #config-form {
  display: flex;
  flex-direction: column;
  flex: 1;
  height: 0;
  min-height: 0;
}

.tab-container {
  flex: 1 1 0;
  min-height: 0;
  height: 0;
  overflow-y: auto;
  padding: 12px 8px 12px 0;
  position: relative;
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
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.page .desc {
  margin: 0;
  color: var(--ba-muted);
  line-height: 1.65;
  font-size: 14px;
}

.list-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.upload-btn,
.reset-btn,
.admin-btn,
.update-btn {
  display: inline-flex;
  align-items: center;
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--ba-blue-line);
  background: linear-gradient(180deg, #ffffff 0%, var(--ba-blue-soft) 100%);
  color: #0f4d88;
  font-size: 14px;
  font-weight: 800;
  cursor: pointer;
  box-shadow: 0 6px 12px rgba(32, 96, 150, 0.12);
  transition: all 0.2s ease;
}

.upload-btn:hover,
.reset-btn:hover,
.admin-btn:hover,
.update-btn:hover:not(:disabled) {
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
  font-size: 12px;
  font-weight: 700;
  padding: 4px 10px;
  border-radius: 7px;
  border: 1px solid var(--ba-blue-line);
  background: #ffffff;
  color: #245b91;
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

.list-textarea {
  width: 100%;
  height: 200px;
  min-height: 120px;
  border: 1px solid #bdd4eb;
  border-radius: 8px;
  padding: 12px;
  font-size: 15px;
  resize: vertical;
  line-height: 1.6;
  font-family: inherit;
  background: #ffffff;
  color: var(--ba-ink);
}

.list-textarea:focus {
  outline: none;
  border-color: var(--ba-blue);
  box-shadow: 0 0 0 3px rgba(18, 132, 255, 0.16);
}

.page label {
  display: block;
  margin: 10px 0;
  font-size: 14px;
  color: var(--ba-ink);
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
  border: 1px solid #bdd4eb;
  border-radius: 8px;
  padding: 10px 12px;
  font-size: 15px;
  background: #ffffff;
  color: var(--ba-ink);
  transition: all 160ms ease;
}

.page input[type="number"]:focus,
.page input[type="text"]:focus,
.page textarea:focus,
.page select:focus {
  outline: none;
  border-color: var(--ba-blue);
  box-shadow: 0 0 0 3px rgba(18, 132, 255, 0.16);
}

.page input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--ba-blue);
}

.page .row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.save-btn {
  margin-top: 20px;
  width: 100%;
  min-height: 46px;
  border: 1px solid #e3b900;
  border-radius: 10px;
  padding: 13px 14px;
  color: #3b2b00;
  font-size: 16px;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(180deg, #fff27a 0%, var(--ba-yellow) 52%, var(--ba-yellow-strong) 100%);
  box-shadow: 0 10px 18px rgba(197, 144, 0, 0.22);
  transition: all 0.2s ease;
}

.save-btn:hover {
  background: linear-gradient(180deg, #fff79d 0%, #ffe060 58%, #ffc928 100%);
}

.save-btn:active {
  transform: translateY(1px);
  box-shadow: inset 0 2px 4px rgba(122, 84, 0, 0.18);
}

.update-card {
  margin-top: 0px;
  border: 1px solid var(--ba-line);
  border-radius: 12px;
  padding: 14px 16px;
  background: #ffffff;
}

.update-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.update-btn {
  color: #ffffff;
  border-color: var(--ba-blue-strong);
  background: linear-gradient(180deg, #42b2ff 0%, var(--ba-blue-strong) 100%);
}

.update-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, #62c2ff 0%, #0877ef 100%);
}

.update-status {
  font-size: 13px;
  font-weight: 600;
  color: var(--ba-muted);
}

.update-status.status-update { color: #1b5fd1; }
.update-status.status-ok { color: #2f7d4b; }
.update-status.status-error { color: #c24040; }

.update-detail {
  margin-top: 10px;
  border: 1px solid var(--ba-line);
  border-radius: 8px;
  padding: 10px 12px;
  background: #ffffff;
  font-size: 13px;
  color: var(--ba-muted);
  white-space: pre-wrap;
}

.admin-block {
  margin-top: 18px;
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
  margin: 0 0 8px;
  font-weight: 700;
  color: #0d3a67;
  font-size: 15px;
}

.admin-hint {
  margin: 6px 0 10px;
  font-size: 13px;
  color: var(--ba-muted);
  line-height: 1.55;
}

.table-wrapper {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid var(--ba-line);
  border-radius: 10px;
  background: #ffffff;
}

.student-table {
  width: 100%;
  border-collapse: collapse;
  text-align: left;
}

.student-table th {
  padding: 10px 14px;
  background: linear-gradient(180deg, #eaf6ff 0%, #dceeff 100%);
  color: #174a78;
  font-weight: 600;
  border-bottom: 1px solid var(--ba-blue-line);
  position: sticky;
  top: 0;
  z-index: 1;
}

.student-table td {
  padding: 10px 14px;
  border-bottom: 1px solid #e1e9f2;
  color: var(--ba-ink);
}

.student-table tbody tr:hover {
  background: #f5fbff;
}

.weight-slider {
  vertical-align: middle;
  width: 120px;
  accent-color: var(--ba-blue);
}

.weight-val {
  display: inline-block;
  width: 30px;
  margin-left: 8px;
  font-size: 13px;
  color: #1c6dae;
  font-weight: 700;
  vertical-align: middle;
}

.del-svg-btn {
  background: none;
  border: none;
  color: #d44747;
  cursor: pointer;
  padding: 6px;
  border-radius: 50%;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;
}

.del-svg-btn:hover {
  background: #fff0f0;
}

.log-header {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
  padding-bottom: 13px;
  border-bottom: 1px solid var(--ba-line);
}

.log-title-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.log-header h2 {
  margin: 0;
  font-size: 19px;
  color: #0c315f;
}

.log-list {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column-reverse;
  gap: 10px;
  padding: 14px 6px 0 0;
  overflow: auto;
}

.log-item {
  display: grid;
  grid-template-columns: 64px 1fr;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  background: #ffffff;
  border: 1px solid var(--ba-line);
  font-size: 13px;
}

.log-item.log-info { border-left: 4px solid var(--ba-blue); }
.log-item.log-success {
  border-left: 4px solid #3cb878;
  background: #f5fff9;
}
.log-item.log-error {
  border-left: 4px solid #e05454;
  background: #fff7f7;
}

.log-time {
  font-variant-numeric: tabular-nums;
  color: #8094aa;
}

.log-text {
  color: var(--ba-ink);
  word-break: break-word;
  overflow-wrap: anywhere;
}

.log-empty {
  padding: 12px;
  color: var(--ba-muted);
  text-align: center;
  background: #f8fcff;
  border: 1px dashed var(--ba-blue-line);
  border-radius: 8px;
}

@media (max-width: 900px) {
  .page { padding: 14px; }
  .layout {
    grid-template-columns: 1fr;
    grid-template-rows: minmax(0, 1fr) 320px;
  }
  .page .tabs { overflow-x: auto; }
  .page .tab-btn { min-width: 112px; }
}
</style>

