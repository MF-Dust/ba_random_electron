<template>
  <main class="page">
    <section class="panel">
      <div class="header">
        <h1>BA-Random Web配置页</h1>
        <p class="hint">老师可以在这里配置BA-Random的各项功能哦！</p>
      </div>

      <div class="tabs">
        <button type="button" class="tab-btn" :class="{ active: activeTab === 'list' }" @click="switchTab('list')">花名册导入</button>
        <button type="button" class="tab-btn" :class="{ active: activeTab === 'students' }" @click="switchTab('students')">花名册管理</button>
        <button type="button" class="tab-btn" :class="{ active: activeTab === 'floating' }" @click="switchTab('floating')">抽取悬浮按钮</button>
        <button type="button" class="tab-btn" :class="{ active: activeTab === 'pickCount' }" @click="switchTab('pickCount')">抽取人数选择窗口</button>
        <button type="button" class="tab-btn" :class="{ active: activeTab === 'web' }" @click="switchTab('web')">Web配置网络服务</button>
      </div>

      <form id="config-form" @submit.prevent="saveConfig">
        <div class="tab-container">
          <transition :name="transitionName" mode="out-in">
            <div class="tab-content" v-if="activeTab === 'list'" key="list">
              <div class="list-manager">
                <p class="desc">老师可以手动输入名单（每行一个），或者点击下方按钮导入CSV/TXT文件自动解析！</p>
                <div class="list-actions">
                  <label class="upload-btn">
                    <span>📂 导入文件</span>
                    <input type="file" accept=".txt,.csv" @change="handleFileUpload" style="display: none;" />
                  </label>
                  <span class="count-badge">当前导入人数：{{ config.studentList.length }}</span>
                </div>
                <textarea 
                  v-model="rawListText" 
                  class="list-textarea" 
                  placeholder="请输入名单，每行一个。例如：
早濑优香
小鸟游星野
空崎日奈"
                  @input="syncTextToList"
                ></textarea>
              </div>
            </div>

            <div class="tab-content" v-else-if="activeTab === 'students'" key="students">
              <div class="student-manager">
                <p class="desc">老师可以在这里管理当前名单中人员及抽取权重，默认权重为1.0。权重越高，被抽取到的概率越大!</p>
                <div class="student-list table-wrapper">
                  <div v-if="config.studentList.length === 0" class="empty-tips-text">暂时没有名单哦~请先在“名单导入”中输入。</div>
                  <div v-if="config.studentList.length === 0" class="empty-tips-arona">
                  <img v-if="config.studentList.length === 0" src="/image/Arona_Empty.png" alt="Arona Empty" class="empty-tips-arona-img" />
                  </div>
                  <table class="student-table" v-else>
                    <thead>
                      <tr>
                        <th class="col-name">学生姓名</th>
                        <th class="col-weight">权重 (0.0 - 2.0)</th>
                        <th class="col-action">删除</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="(student, index) in config.studentList" :key="index">
                        <td class="col-name">{{ student.name }}</td>
                        <td class="col-weight">
                          <input 
                            type="range" 
                            class="weight-slider"
                            v-model.number="student.weight" 
                            min="0" max="2" step="0.1" 
                            @change="syncListToText" 
                          />
                          <span class="weight-val">{{ Number(student.weight).toFixed(1) }}</span>
                        </td>
                        <td class="col-action">
                          <button type="button" class="del-svg-btn" @click="removeStudent(index)" title="删除">
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
                  <button type="button" class="reset-btn" @click="resetWeights">重置所有学生的权重为1.0</button>
                </div>
              </div>
            </div>

            <div class="tab-content" v-else-if="activeTab === 'floating'" key="floating">
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

            <div class="tab-content" v-else-if="activeTab === 'pickCount'" key="pickCount">
              <label class="inline">
                <input type="checkbox" v-model="config.pickCountDialog.defaultPlayMusic" />
                默认播放背景音乐（注意！教学环境下可能并不适宜）
              </label>
              <label>
                背景变暗程度，百分数值（0-100）
                <input type="number" v-model.number="config.pickCountDialog.backgroundDarknessPercent" min="0" max="100" required />
              </label>
              <label>
                默认人数（1-10）
                <input type="number" v-model.number="config.pickCountDialog.defaultCount" min="1" max="10" required />
              </label>
            </div>

            <div class="tab-content" v-else-if="activeTab === 'web'" key="web">
              <p class="desc">在此修改BA-Random——Electron的网络相关配置，应用冷重启后生效。通常情况下老师不需要配置这里的选项</p>
              <label>
                Web配置界面的端口（1-65535）
                <input type="number" v-model.number="config.webConfig.port" min="1" max="65535" required />
              </label>
            </div>
          </transition>
        </div>

        <button type="submit" class="save-btn">保存配置</button>
      </form>
    </section>
  </main>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import axios from 'axios'

const tabs = ['list', 'students', 'floating', 'pickCount', 'web']
const activeTab = ref('list')
const transitionName = ref('slide-left')

const switchTab = (tab) => {
  const currentIndex = tabs.indexOf(activeTab.value)
  const nextIndex = tabs.indexOf(tab)
  transitionName.value = nextIndex > currentIndex ? 'slide-left' : 'slide-right'
  activeTab.value = tab
}

const apiBase = '/api'

const config = ref({
  studentList: [],
  floatingButton: {
    sizePercent: 100,
    transparencyPercent: 100,
    alwaysOnTop: true,
    position: {
      x: null,
      y: null
    }
  },
  pickCountDialog: {
    defaultPlayMusic: true,
    backgroundDarknessPercent: 50,
    defaultCount: 1
  },
  webConfig: {
    port: 21219
  }
})

const rawListText = ref('')

const syncTextToList = () => {
  const names = rawListText.value
    .split(/[\r\n]+/)
    .flatMap(line => line.split(','))
    .map(name => name.trim())
    .filter(name => name)

  const uniqueNames = Array.from(new Set(names))
  const existingMap = new Map(config.value.studentList.map(s => [s.name, s.weight]))
  
  config.value.studentList = uniqueNames.map(name => ({
    name,
    weight: existingMap.has(name) ? existingMap.get(name) : 1.0
  }))
}

const syncListToText = () => {
  rawListText.value = config.value.studentList.map(s => s.name).join('\n')
}

const removeStudent = (index) => {
  config.value.studentList.splice(index, 1)
  syncListToText()
}

const resetWeights = () => {
  config.value.studentList.forEach(s => { s.weight = 1.0 })
}

const handleFileUpload = (event) => {
  const file = event.target.files[0]
  if (!file) return

  const reader = new FileReader()
  reader.readAsText(file, 'utf-8')
  reader.onload = (e) => {
    const text = e.target.result
    const lines = text
      .split(/[\r\n]+/)
      .map(line => line.trim())
      .filter(line => line)
    
    rawListText.value = lines.join('\n')
    syncTextToList()
    event.target.value = ''
  }
}

const maybeNumber = (value) => {
  if (value === '' || value === null || value === undefined) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
}

const fetchConfig = async () => {
  try {
    const response = await axios.get(`${apiBase}/config`)
    config.value = response.data
    rawListText.value = (config.value.studentList || []).map(s => s.name).join('\n')
  } catch (error) {
    console.error('加载配置失败:', error)
    window.alert('配置页面初始化失败。')
  }
}

const saveConfig = async () => {
  try {
    syncTextToList()
    const payload = {
      studentList: config.value.studentList,
      floatingButton: {
        sizePercent: Number(config.value.floatingButton.sizePercent),
        transparencyPercent: Number(config.value.floatingButton.transparencyPercent),
        alwaysOnTop: Boolean(config.value.floatingButton.alwaysOnTop),
        position: {
          x: maybeNumber(config.value.floatingButton.position.x),
          y: maybeNumber(config.value.floatingButton.position.y)
        }
      },
      pickCountDialog: {
        defaultPlayMusic: Boolean(config.value.pickCountDialog.defaultPlayMusic),
        backgroundDarknessPercent: Number(config.value.pickCountDialog.backgroundDarknessPercent),
        defaultCount: Number(config.value.pickCountDialog.defaultCount)
      },
      webConfig: {
        port: Number(config.value.webConfig.port)
      }
    }

    await axios.post(`${apiBase}/config`, payload)

    window.alert('配置已保存并生效。')
  } catch (error) {
    console.error('保存配置失败:', error)
    window.alert('保存失败，请检查输入内容。')
  }
}

onMounted(() => {
  fetchConfig()
})
</script>

<style scoped>
* {
  box-sizing: border-box;
}

.page {
  min-height: 100vh;
  padding: 24px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  font-family: "Microsoft YaHei UI", "PingFang SC", sans-serif;
  background:#c1e6ff;
  color: #133053;
}

.panel {
  width: min(760px, 96vw);
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.7);
  border-radius: 10px;
  box-shadow: 0 20px 40px rgba(12, 36, 68, 0.22);
  padding: 24px;
}

h1 {
  margin: 0;
  font-size: 30px;
  letter-spacing: 1px;
  text-align: center;
}

.hint {
  margin-top: 8px;
  color: #355985;
  text-align: center;
}

.tabs {
  display: flex;
  margin: 24px 0 20px;
  border-bottom: 2px solid #aac2e0;
}

.tab-btn {
  flex: 1;
  background: none;
  border: none;
  font-size: 16px;
  font-weight: 600;
  color: #355985;
  padding: 12px 0;
  cursor: pointer;
  transition: all 0.2s ease;
  border-bottom: 3px solid transparent;
  margin-bottom: -2px;
}

.tab-btn:hover {
  background: rgba(170, 194, 224, 0.2);
}

.tab-btn.active {
  color: #133053;
  border-bottom-color: #2c69b2;
}

.tab-content {
  min-height: 220px;
  padding: 10px 0;
}

.list-manager {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.desc {
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
  border: 1px solid #aac2e0;
  border-radius: 10px;
  padding: 12px;
  font-size: 15px;
  resize: vertical;
  line-height: 1.6;
  font-family: inherit;
}

.list-textarea:focus {
  outline: none;
  border-color: #2c69b2;
}

label {
  display: block;
  margin: 10px 0;
  font-size: 14px;
}

.inline {
  display: flex;
  align-items: center;
  gap: 8px;
}

input[type="number"] {
  width: 100%;
  margin-top: 6px;
  border: 1px solid #aac2e0;
  border-radius: 10px;
  padding: 10px 12px;
  font-size: 15px;
}

input[type="checkbox"] {
  width: 16px;
  height: 16px;
}

.row {
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
  color: #000000;
  font-size: 17px;
  font-weight: 700;
  cursor: pointer;
  background: #eae72192;
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

/* 滑动切换动画 */
.tab-container {
  position: relative;
  overflow: hidden;
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