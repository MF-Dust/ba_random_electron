import { ref } from 'vue'

const defaultTaskName = 'Blue Random (Admin)'

const createDefaultConfig = () => ({
  studentList: [],
  allowRepeatDraw: true,
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
  pickResultDialog: {
    defaultPlayGachaSound: true,
    gachaSoundVolume: 0.6
  },
  webConfig: {
    port: 21219,
    adminTopmostEnabled: false,
    adminAutoStartEnabled: false,
    adminAutoStartPath: '',
    adminAutoStartTaskName: defaultTaskName
  }
})

const maybeNumber = (value) => {
  if (value === '' || value === null || value === undefined) return null
  const n = Number(value)
  return Number.isFinite(n) ? n : null
}

export function useAppConfig(appApi, addLog) {
  const config = ref(createDefaultConfig())
  const isDebugMode = ref(false)
  const isAdmin = ref(false)
  const appVersion = ref('0.0.0')
  const defaultExePath = ref('')

  const applyDefaultAutoStartPath = () => {
    if (!config.value || !config.value.webConfig) return
    if (!config.value.webConfig.adminAutoStartPath && defaultExePath.value) {
      config.value.webConfig.adminAutoStartPath = defaultExePath.value
    }
  }

  const fetchConfig = async (rawListText) => {
    try {
      config.value = await appApi.getConfig()
      if (rawListText) {
        rawListText.value = (config.value.studentList || []).map(s => s.name).join('\n')
      }
      applyDefaultAutoStartPath()
      addLog('info', '配置已加载')
    } catch (error) {
      console.error('加载配置失败:', error)
      addLog('error', '加载配置失败，请检查应用后端是否启动')
      window.alert('配置页面初始化失败。')
    }
  }

  const fetchAppInfo = async () => {
    try {
      const response = await appApi.getAppInfo()
      isDebugMode.value = Boolean(response && response.isDebugMode)
      isAdmin.value = Boolean(response && response.isAdmin)
      appVersion.value = response && response.version ? response.version : '0.0.0'
      defaultExePath.value = response && response.exePath ? response.exePath : ''
      applyDefaultAutoStartPath()
    } catch (_error) {
      isDebugMode.value = false
      isAdmin.value = false
      appVersion.value = '0.0.0'
      defaultExePath.value = ''
    }
  }

  const saveConfig = async (syncTextToList) => {
    try {
      await syncTextToList({ updateText: true })
      const payload = {
        studentList: config.value.studentList,
        allowRepeatDraw: Boolean(config.value.allowRepeatDraw),
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
        pickResultDialog: {
          defaultPlayGachaSound: Boolean(config.value.pickResultDialog.defaultPlayGachaSound),
          gachaSoundVolume: Number(config.value.pickResultDialog.gachaSoundVolume)
        },
        webConfig: {
          port: Number(config.value.webConfig.port || 21219),
          adminTopmostEnabled: Boolean(config.value.webConfig.adminTopmostEnabled),
          adminAutoStartEnabled: Boolean(config.value.webConfig.adminAutoStartEnabled),
          adminAutoStartPath: String(config.value.webConfig.adminAutoStartPath || ''),
          adminAutoStartTaskName: String(config.value.webConfig.adminAutoStartTaskName || defaultTaskName)
        }
      }

      await appApi.saveConfig(payload)
      addLog('success', '配置已保存并生效')
      window.alert('配置已保存并生效。')
    } catch (error) {
      console.error('保存配置失败:', error)
      addLog('error', '保存失败，请检查输入内容')
      window.alert('保存失败，请检查输入内容。')
    }
  }

  const requestAdminElevation = async () => {
    try {
      const response = await appApi.requestAdminElevation()
      addLog(response.ok ? 'info' : 'error', response.message || '已发送管理员权限请求')
      window.alert(response.message || '已发送管理员权限请求。')
    } catch (error) {
      console.error('申请管理员权限失败:', error)
      const message = error?.message || '申请管理员权限失败'
      addLog('error', message)
      window.alert(`${message}，请查看日志。`)
    }
  }

  const createAdminStartupTask = async () => {
    try {
      const fallbackPath = defaultExePath.value || ''
      const payload = {
        exePath: String(config.value.webConfig.adminAutoStartPath || fallbackPath).trim(),
        taskName: String(config.value.webConfig.adminAutoStartTaskName || defaultTaskName).trim()
      }
      if (!payload.exePath) {
        window.alert('请先填写可执行文件路径。')
        return
      }
      const response = await appApi.createAdminStartupTask(payload.exePath, payload.taskName)
      addLog(response.ok ? 'success' : 'error', response.message || '计划任务已创建或更新')
      window.alert(response.message || '计划任务已创建或更新。')
    } catch (error) {
      console.error('创建计划任务失败:', error)
      addLog('error', '创建计划任务失败')
      window.alert('创建计划任务失败，请查看日志。')
    }
  }

  return {
    config,
    isDebugMode,
    isAdmin,
    appVersion,
    defaultExePath,
    fetchConfig,
    fetchAppInfo,
    saveConfig,
    requestAdminElevation,
    createAdminStartupTask
  }
}
