<template>
  <main class="page">
    <section class="panel">
      <h1>BA-Random Web配置页</h1>
      <p class="hint">老师可以在这里配置BA-Random的各项功能哦！</p>

      <form id="config-form" @submit.prevent="saveConfig">
        <h2>悬浮点名按钮</h2>
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

        <h2>人数选择窗口</h2>
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

        <h2>网页配置服务</h2>
        <label>
          端口（1-65535）
          <input type="number" v-model.number="config.webConfig.port" min="1" max="65535" required />
        </label>

        <button type="submit" class="save-btn">保存配置</button>
      </form>
    </section>
  </main>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

// 统一走前端相对路径，开发模式由 Vite dev server proxy 代理到 21219，打包后由后端直出
const apiBase = '/api'

const config = ref({
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

const maybeNumber = (value) => {
  if (value === '' || value === null || value === undefined) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
}

const fetchConfig = async () => {
  try {
    const response = await axios.get(`${apiBase}/config`)
    config.value = response.data
  } catch (error) {
    console.error('加载配置失败:', error)
    window.alert('配置页面初始化失败。')
  }
}

const saveConfig = async () => {
  try {
    const payload = {
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
  background: radial-gradient(circle at 20% 0%, #e8f5ff, #c7dcf5 40%, #afc7e9 100%);
  color: #133053;
}

.panel {
  width: min(760px, 96vw);
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.7);
  border-radius: 20px;
  box-shadow: 0 20px 40px rgba(12, 36, 68, 0.22);
  padding: 24px;
}

h1 {
  margin: 0;
  font-size: 30px;
  letter-spacing: 1px;
}

.hint {
  margin-top: 8px;
  color: #355985;
}

h2 {
  margin: 22px 0 10px;
  font-size: 20px;
  border-left: 4px solid #2c69b2;
  padding-left: 10px;
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
</style>